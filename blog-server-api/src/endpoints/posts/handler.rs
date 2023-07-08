use super::request_content::PostsRequestContent;
use super::response_content_failure::PostsResponseContentFailure;
use super::response_content_failure::PostsResponseContentFailure::*;
use super::response_content_success::PostsResponseContentSuccess;
use crate::extensions::Resolve;
use blog_server_services::traits::post_service::PostService;
use screw_api::request::ApiRequest;
use screw_api::response::ApiResponse;
use std::sync::Arc;
use tokio::join;

async fn handler(
    offset: Option<i64>,
    limit: Option<i64>,
    post_service: Arc<Box<dyn PostService>>,
) -> Result<PostsResponseContentSuccess, PostsResponseContentFailure> {
    let offset = offset.unwrap_or(0).max(0);
    let limit = limit.unwrap_or(50).max(0).min(50);

    let (posts_result, total_result) = join!(
        post_service.posts(&offset, &limit),
        post_service.posts_count(),
    );

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

    Ok(PostsResponseContentSuccess {
        posts,
        total,
        offset,
        limit,
    })
}

pub async fn http_handler<Extensions>(
    request: ApiRequest<PostsRequestContent, Extensions>,
) -> ApiResponse<PostsResponseContentSuccess, PostsResponseContentFailure>
where
    Extensions: Resolve<Arc<Box<dyn PostService>>>,
{
    ApiResponse::from(
        handler(
            request.content.offset,
            request.content.limit,
            request.content.post_service,
        )
        .await,
    )
}
