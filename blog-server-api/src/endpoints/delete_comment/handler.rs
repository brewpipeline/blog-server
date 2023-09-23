use super::request_content::DeleteCommentRequestContent;
use super::response_content_failure::DeleteCommentResponseContentFailure;
use super::response_content_failure::DeleteCommentResponseContentFailure::*;
use super::response_content_success::DeleteCommentResponseContentSuccess;

pub async fn http_handler(
    (DeleteCommentRequestContent {
        id,
        comment_service,
        auth_author_future,
    },): (DeleteCommentRequestContent,),
) -> Result<DeleteCommentResponseContentSuccess, DeleteCommentResponseContentFailure> {
    let id = id.parse::<u64>().map_err(|e| IncorrectIdFormat {
        reason: e.to_string(),
    })?;

    if id == 0 {
        return Err(IncorrectIdFormat {
            reason: String::from("should not be equal to zero"),
        });
    }

    let author = auth_author_future.await.map_err(|e| Unauthorized {
        reason: e.to_string(),
    })?;

    if author.base.blocked == 1 {
        return Err(EditingForbidden);
    }

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
