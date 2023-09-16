use super::request_content::DeletePostRequestContent;
use super::response_content_failure::DeletePostResponseContentFailure;
use super::response_content_failure::DeletePostResponseContentFailure::*;
use super::response_content_success::DeletePostResponseContentSuccess;

pub async fn http_handler(
    (DeletePostRequestContent {
        id,
        post_service,
        auth_author_future,
    },): (DeletePostRequestContent,),
) -> Result<DeletePostResponseContentSuccess, DeletePostResponseContentFailure> {
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

    let post = post_service
        .post_by_id(&id)
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?
        .ok_or(NotFound)?;

    if !(post.base.author_id == author.id || author.base.editor == 1) {
        return Err(if post.base.published == 1 {
            EditingForbidden
        } else {
            NotFound
        });
    }

    post_service
        .delete_post_by_id(&id)
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?;

    Ok(DeletePostResponseContentSuccess)
}
