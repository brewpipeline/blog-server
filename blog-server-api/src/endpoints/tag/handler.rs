use super::request_content::TagRequestContent;
use super::response_content_failure::TagResponseContentFailure;
use super::response_content_failure::TagResponseContentFailure::*;
use super::response_content_success::TagResponseContentSuccess;

pub async fn http_handler(
    (TagRequestContent { id, post_service },): (TagRequestContent,),
) -> Result<TagResponseContentSuccess, TagResponseContentFailure> {
    let id = id.parse::<u64>().map_err(|e| IncorrectIdFormat {
        reason: e.to_string(),
    })?;

    if id == 0 {
        return Err(IncorrectIdFormat {
            reason: String::from("should not be equal to zero"),
        });
    }

    let tag = post_service
        .tag_by_id(&id)
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?
        .ok_or(NotFound)?;

    Ok(tag.into())
}
