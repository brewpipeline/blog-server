use super::request_content::{CreatePostRequestContent, CreatePostRequestData};
use super::response_content_failure::CreatePostContentFailure;
use super::response_content_failure::CreatePostContentFailure::*;
use super::response_content_success::CreatePostContentSuccess;
use crate::extensions::Resolve;
use blog_server_services::traits::post_service::PostService;
use screw_api::request::ApiRequest;
use screw_api::response::ApiResponse;
use screw_components::dyn_result::DResult;
use std::sync::Arc;

async fn handler(
    new_post_data: DResult<CreatePostRequestData>,
    post_service: Arc<Box<dyn PostService>>,
) -> Result<CreatePostContentSuccess, CreatePostContentFailure> {
    let CreatePostRequestData {
        post_id,
        post_content,
    } = new_post_data.map_err(|e| ValidationError {
        reason: e.to_string(),
    })?;

    let test = post_service
        .post_by_id(&post_id)
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?
        .ok_or(AlreadyExists)?;

    Ok(test.into())
}

pub async fn http_handler<Extensions>(
    request: ApiRequest<CreatePostRequestContent, Extensions>,
) -> ApiResponse<CreatePostContentSuccess, CreatePostContentFailure>
where
    Extensions: Resolve<Arc<Box<dyn PostService>>>,
{
    ApiResponse::from(handler(request.content.new_post_data, request.content.post_service).await)
}
