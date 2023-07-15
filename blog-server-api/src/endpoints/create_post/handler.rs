use super::request_content::CreatePostRequestContent;
use super::response_content_failure::CreatePostContentFailure;
use super::response_content_failure::CreatePostContentFailure::*;
use super::response_content_success::CreatePostContentSuccess;
use crate::entities::CreatePost;
use crate::extensions::Resolve;
use blog_server_services::traits::post_service::PostService;
use screw_api::request::ApiRequest;
use screw_api::response::ApiResponse;
use screw_components::dyn_result::DResult;
use std::sync::Arc;

async fn handler(
    new_post_data: DResult<CreatePost>,
    post_service: Arc<Box<dyn PostService>>,
) -> Result<CreatePostContentSuccess, CreatePostContentFailure> {
    let base_post = new_post_data
        .map_err(|e| ValidationError {
            reason: e.to_string(),
        })?
        .into(1);

    let inserted_id = post_service
        .create_post(&base_post)
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?;

    let created_post = post_service
        .post_by_id(&inserted_id)
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?
        .ok_or(InsertFailed)?;

    Ok(created_post.into())
}

pub async fn http_handler<Extensions>(
    request: ApiRequest<CreatePostRequestContent, Extensions>,
) -> ApiResponse<CreatePostContentSuccess, CreatePostContentFailure>
where
    Extensions: Resolve<Arc<Box<dyn PostService>>>,
{
    ApiResponse::from(handler(request.content.new_post_data, request.content.post_service).await)
}
