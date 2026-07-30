use blog_server_services::traits::author_service::Author;
use validator::Validate;

use super::request_content::CreateCommentRequestContent;
use super::response_content_failure::CreateCommentContentFailure;
use super::response_content_failure::CreateCommentContentFailure::*;
use super::response_content_success::CreateCommentContentSuccess;

pub async fn http_handler(
    (
        author,
        CreateCommentRequestContent {
            new_comment_data,
            comment_service,
        },
    ): (Author, CreateCommentRequestContent),
) -> Result<CreateCommentContentSuccess, CreateCommentContentFailure> {
    let base_comment = new_comment_data.map_err(|e| ValidationError {
        reason: e.to_string(),
    })?;

    base_comment.validate().map_err(|e| ValidationError {
        reason: e.to_string(),
    })?;

    let _ = comment_service
        .create_comment(&From::from((author.id, base_comment)))
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?;

    Ok(CreateCommentContentSuccess)
}
