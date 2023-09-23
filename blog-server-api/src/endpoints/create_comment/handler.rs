use super::request_content::CreateCommentRequestContent;
use super::response_content_failure::CreateCommentContentFailure;
use super::response_content_failure::CreateCommentContentFailure::*;
use super::response_content_success::CreateCommentContentSuccess;

pub async fn http_handler(
    (CreateCommentRequestContent {
        new_comment_data,
        comment_service,
        auth_author_future,
    },): (CreateCommentRequestContent,),
) -> Result<CreateCommentContentSuccess, CreateCommentContentFailure> {
    let author = auth_author_future.await.map_err(|e| Unauthorized {
        reason: e.to_string(),
    })?;

    if author.base.blocked == 1 {
        return Err(CreatingForbidden);
    }

    let base_comment = new_comment_data.map_err(|e| ValidationError {
        reason: e.to_string(),
    })?;

    if base_comment.content.is_empty() {
        return Err(ValidationError {
            reason: "comment should not be empty".to_owned(),
        });
    }

    let _ = comment_service
        .create_comment(&From::from((author.id, base_comment)))
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?;

    Ok(CreateCommentContentSuccess)
}
