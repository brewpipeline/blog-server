use super::request_content::DeletePostRequestContent;
use super::response_content_failure::DeletePostResponseContentFailure;
use super::response_content_failure::DeletePostResponseContentFailure::*;
use super::response_content_success::DeletePostResponseContentSuccess;

pub async fn http_handler(
    (DeletePostRequestContent { id, post_service },): (DeletePostRequestContent,),
) -> Result<DeletePostResponseContentSuccess, DeletePostResponseContentFailure> {
    let id = id.parse::<u64>().map_err(|e| IncorrectIdFormat {
        reason: e.to_string(),
    })?;

    if id == 0 {
        return Err(IncorrectIdFormat {
            reason: String::from("should not be equal to zero"),
        });
    }

    let _ = post_service
        .post_by_id(&id)
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?
        .ok_or(NotFound)?;

    post_service
        .delete_post_by_id(&id)
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?;

    Ok(DeletePostResponseContentSuccess)
}
