use super::request_content::CreatePostRequestContent;
use super::response_content_failure::CreatePostContentFailure;
use super::response_content_failure::CreatePostContentFailure::*;
use super::response_content_success::CreatePostContentSuccess;

pub async fn http_handler(
    (CreatePostRequestContent {
        new_post_data,
        post_service,
    },): (CreatePostRequestContent,),
) -> Result<CreatePostContentSuccess, CreatePostContentFailure> {
    let base_post = new_post_data
        .map_err(|e| ValidationError {
            reason: e.to_string(),
        })?
        .into();

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
