use blog_generic::entities::PublishType;
use blog_generic::events::NewPostPublished;
use blog_server_services::traits::author_service::Author;
use validator::Validate;

use super::request_content::CreatePostRequestContent;
use super::response_content_failure::CreatePostContentFailure;
use super::response_content_failure::CreatePostContentFailure::*;
use super::response_content_success::CreatePostContentSuccess;

pub async fn http_handler(
    (
        author,
        CreatePostRequestContent {
            new_post_data: base_post,
            post_service,
            entity_post_service,
            new_post_service,
        },
    ): (Author, CreatePostRequestContent),
) -> Result<CreatePostContentSuccess, CreatePostContentFailure> {
    if let Some(err) = base_post.validate().err() {
        return Err(ValidationError {
            reason: err.to_string(),
        }
        .into());
    }

    if author.base.editor == 0 && base_post.publish_type.is_published() {
        return Err(ValidationError {
            reason: "publishing not allowed for you".to_owned(),
        }
        .into());
    }

    let tag_titles: Vec<String> = base_post.tags.to_owned();

    let inserted_id = post_service
        .create_post(&From::from((author.id, base_post)))
        .await?;

    let post_tags = post_service.create_tags(tag_titles).await?;

    post_service
        .merge_post_tags(&inserted_id, post_tags)
        .await?;

    let created_post = post_service
        .post_by_id(&inserted_id)
        .await?
        .ok_or(InsertFailed)?;

    let created_post_entity = entity_post_service
        .posts_entities(vec![created_post])
        .await?
        .remove(0);

    if created_post_entity.publish_type == PublishType::Published {
        let new_post_published = NewPostPublished {
            blog_user_id: created_post_entity.author.id,
            post_sub_url: format!(
                "/post/{}/{}",
                created_post_entity.slug, created_post_entity.id
            ),
        };
        tokio::spawn(async move { new_post_service.publish(new_post_published).await });
    }

    Ok(created_post_entity.into())
}
