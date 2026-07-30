use blog_server_services::traits::author_service::Author;

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

    if base_comment.content.is_empty() {
        return Err(ValidationError {
            reason: "comment should not be empty".to_owned(),
        }
        .into());
    }

    if base_comment.content.chars().count() > 500 {
        return Err(ValidationError {
            reason: "comment should not less then 500 symbols".to_owned(),
        }
        .into());
    }

    let _ = comment_service
        .create_comment(&From::from((author.id, base_comment)))
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?;

    Ok(CreateCommentContentSuccess)
}
