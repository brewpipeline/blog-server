use super::request_content::PostRequestContent;
use super::response_content_failure::PostResponseContentFailure;
use super::response_content_failure::PostResponseContentFailure::*;
use super::response_content_success::PostResponseContentSuccess;
use crate::extensions::Resolve;
use blog_server_services::traits::post_service::PostService;
use screw_api::request::ApiRequest;
use screw_api::response::ApiResponse;
use std::sync::Arc;

async fn handler(
    slug: String,
    post_service: Arc<Box<dyn PostService>>,
) -> Result<PostResponseContentSuccess, PostResponseContentFailure> {
    if slug.is_empty() {
        return Err(SlugEmpty);
    }

    let post = post_service
        .post_by_slug(&slug)
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?
        .ok_or(NotFound)?;

    Ok(post.into())
}

pub async fn http_handler<Extensions>(
    request: ApiRequest<PostRequestContent, Extensions>,
) -> ApiResponse<PostResponseContentSuccess, PostResponseContentFailure>
where
    Extensions: Resolve<Arc<Box<dyn PostService>>>,
{
    ApiResponse::from(handler(request.content.slug, request.content.post_service).await)
}
