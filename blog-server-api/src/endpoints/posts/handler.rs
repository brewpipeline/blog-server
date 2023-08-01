use blog_generic::entities::{BaseItemsContainer, PostsContainer};

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

    let posts = posts_result
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?
        .into_iter()
        .map(|a| a.into())
        .collect();

    let total = total_result.map_err(|e| DatabaseError {
        reason: e.to_string(),
    })?;

    Ok(PostsContainer {
        posts,
        base: BaseItemsContainer {
            total,
            offset,
            limit,
        },
    }
    .into())
}
