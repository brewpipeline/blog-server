use super::request_content::PostRequestContent;
use super::response_content_failure::PostResponseContentFailure;
use super::response_content_failure::PostResponseContentFailure::*;
use super::response_content_success::PostResponseContentSuccess;

pub async fn http_handler(
    (PostRequestContent {
        id,
        post_service,
        entity_post_service,
    },): (PostRequestContent,),
) -> Result<PostResponseContentSuccess, PostResponseContentFailure> {
    let id = id.parse::<u64>().map_err(|e| IncorrectIdFormat {
        reason: e.to_string(),
    })?;

    if id == 0 {
        return Err(IncorrectIdFormat {
            reason: String::from("should not be equal to zero"),
        });
    }

    let post = post_service
        .post_by_id(&id)
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?
        .ok_or(NotFound)?;

    let post_entity = entity_post_service
        .posts_entities(vec![post])
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?
        .remove(0);

    Ok(post_entity.into())
}
