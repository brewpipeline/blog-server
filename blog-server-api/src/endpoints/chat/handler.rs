use blog_generic::entities::{ChatAnswer, PublishType};
use blog_server_services::traits::author_service::AuthorService;
use blog_server_services::traits::entity_post_service::EntityPostService;
use blog_server_services::traits::post_service::{PostService, PostsQuery};
use once_cell::sync::Lazy;
use reqwest::Client;
use serde_json::{json, Value};
use std::collections::HashMap;
use tokio::sync::Mutex;

use super::request_content::ChatRequestContent;
use super::response_content_failure::ChatResponseContentFailure;
use super::response_content_failure::ChatResponseContentFailure::*;
use super::response_content_success::ChatResponseContentSuccess;

static SESSION_DATA: Lazy<Mutex<HashMap<String, (u8, Vec<Value>)>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub async fn http_handler(
    (ChatRequestContent {
        question,
        post_service,
        entity_post_service,
        author_service,
        session_key,
    },): (ChatRequestContent,),
) -> Result<ChatResponseContentSuccess, ChatResponseContentFailure> {
    let question = question.map_err(|e| ParamsDecodeError {
        reason: e.to_string(),
    })?;
    let session_key = session_key;

    let posts_total = post_service
        .posts(PostsQuery::offset_and_limit(&0, &0).publish_type(Some(&PublishType::Published)))
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?
        .total_count;
    let posts_list = if posts_total > 0 {
        post_service
            .posts(
                PostsQuery::offset_and_limit(&0, &posts_total)
                    .publish_type(Some(&PublishType::Published)),
            )
            .await
            .map_err(|e| DatabaseError {
                reason: e.to_string(),
            })?
            .posts
    } else {
        vec![]
    };
    let posts_entities = entity_post_service
        .posts_entities(posts_list)
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?;

    let authors_total = author_service
        .authors_count()
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?;
    let authors = if authors_total > 0 {
        author_service
            .authors(&0, &authors_total)
            .await
            .map_err(|e| DatabaseError {
                reason: e.to_string(),
            })?
    } else {
        vec![]
    };

    let posts_context = posts_entities
        .into_iter()
        .map(|p| {
            let author_name = format!(
                "{} {}",
                p.author.first_name.clone().unwrap_or_default(),
                p.author.last_name.clone().unwrap_or_default()
            );
            let author_display = if author_name.trim().is_empty() {
                p.author.slug.clone()
            } else {
                author_name
            };
            let tags = p.joined_tags_string(", ");
            let tags_display = if tags.is_empty() {
                String::new()
            } else {
                format!(" [tags: {}]", tags)
            };
            format!(
                "Post: {} by {}{} - {}",
                p.title, author_display, tags_display, p.summary
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let authors_context = authors
        .into_iter()
        .map(|a| {
            format!(
                "Author {} - {} {}",
                a.base.slug,
                a.base.first_name.unwrap_or_default(),
                a.base.last_name.unwrap_or_default()
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let system_message = json!({
        "role": "system",
        "content": "You strictly answer only about the provided blog posts and authors. If asked anything else, respond with 'I can only answer questions about the blog'. Keep answers under 50 words.",
    });

    let messages = {
        let mut sessions = SESSION_DATA.lock().await;
        let session = sessions
            .entry(session_key.clone())
            .or_insert((0u8, Vec::new()));
        if session.0 >= 5 {
            return Err(SessionLimitReached);
        }
        session.0 += 1;
        if session.1.is_empty() {
            session.1.push(json!({
                "role": "user",
                "content": format!("Posts:\n{}\n\nAuthors:\n{}", posts_context, authors_context),
            }));
        }
        let mut msgs = vec![system_message];
        msgs.extend(session.1.clone());
        let q_msg = json!({ "role": "user", "content": question.question.clone() });
        session.1.push(q_msg.clone());
        msgs.push(q_msg);
        msgs
    };

    let client = Client::new();
    let body = json!({
        "model": "gpt-4o-mini",
        "max_tokens": 100,
        "messages": messages,
    });
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(crate::OPENAI_API_KEY)
        .json(&body)
        .send()
        .await
        .map_err(|e| OpenAiError {
            reason: e.to_string(),
        })?;
    let value: serde_json::Value = response.json().await.map_err(|e| OpenAiError {
        reason: e.to_string(),
    })?;
    let answer = value["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("Couldn't get an answer")
        .to_string();

    {
        let mut sessions = SESSION_DATA.lock().await;
        if let Some(session) = sessions.get_mut(&session_key) {
            session
                .1
                .push(json!({ "role": "assistant", "content": answer.clone() }));
        }
    }

    Ok(ChatAnswer { answer }.into())
}
