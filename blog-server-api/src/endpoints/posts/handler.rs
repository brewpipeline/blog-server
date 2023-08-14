use blog_generic::entities::{PostsContainer, TotalOffsetLimitContainer};

use super::request_content::PostsRequestContent;
use super::response_content_failure::PostsResponseContentFailure;
use super::response_content_failure::PostsResponseContentFailure::*;
use super::response_content_success::PostsResponseContentSuccess;

pub async fn http_handler(
    (PostsRequestContent {
        query,
        offset,
        limit,
        post_service,
        entity_post_service,
    },): (PostsRequestContent,),
) -> Result<PostsResponseContentSuccess, PostsResponseContentFailure> {
    let offset = offset.unwrap_or(0).max(0);
    let limit = limit.unwrap_or(50).max(0).min(50);

    let (posts_result, total_result) = if let Some(query) = query {
        tokio::join!(
            post_service.posts_by_query(&query, &offset, &limit),
            post_service.posts_count_by_query(&query),
        )
    } else {
        tokio::join!(
            post_service.posts(&offset, &limit),
            post_service.posts_count(),
        )
    };

    let posts = posts_result.map_err(|e| DatabaseError {
        reason: e.to_string(),
    })?;

    let total = total_result.map_err(|e| DatabaseError {
        reason: e.to_string(),
    })?;

    let posts_entities = entity_post_service
        .posts_entities(posts)
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?;

    Ok(PostsContainer {
        posts: posts_entities,
        base: TotalOffsetLimitContainer {
            total,
            offset,
            limit,
        },
    }
    .into())
}
