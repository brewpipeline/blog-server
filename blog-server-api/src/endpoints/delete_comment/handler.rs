use blog_server_services::traits::author_service::Author;

use super::request_content::DeleteCommentRequestContent;
use super::response_content_failure::DeleteCommentResponseContentFailure;
use super::response_content_failure::DeleteCommentResponseContentFailure::*;
use super::response_content_success::DeleteCommentResponseContentSuccess;

pub async fn http_handler(
    (
        author,
        DeleteCommentRequestContent {
            id,
            comment_service,
        },
    ): (Author, DeleteCommentRequestContent),
) -> Result<DeleteCommentResponseContentSuccess, DeleteCommentResponseContentFailure> {
    let id = id.parse::<u64>().map_err(|e| IncorrectIdFormat {
        reason: e.to_string(),
    })?;

    let comment = comment_service
        .comment_by_id(&id)
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?
        .ok_or(NotFound)?;

    if !(comment.base.author_id == author.id || author.base.editor == 1) {
        return Err(EditingForbidden);
    }

    comment_service
        .mark_deleted_by_id(&id)
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?;

    Ok(DeleteCommentResponseContentSuccess)
}
