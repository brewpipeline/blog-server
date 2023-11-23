use validator::Validate;

use super::request_content::UpdateSecondaryAuthorRequestContent;
use super::response_content_failure::UpdateSecondaryAuthorContentFailure;
use super::response_content_failure::UpdateSecondaryAuthorContentFailure::*;
use super::response_content_success::UpdateSecondaryAuthorContentSuccess;

pub async fn http_handler(
    (UpdateSecondaryAuthorRequestContent {
        updated_secondary_author_data,
        author_service,
        auth_author_future,
    },): (UpdateSecondaryAuthorRequestContent,),
) -> Result<UpdateSecondaryAuthorContentSuccess, UpdateSecondaryAuthorContentFailure> {
    let author = auth_author_future.await.map_err(|e| Unauthorized {
        reason: e.to_string(),
    })?;

    if author.base.blocked == 1 {
        return Err(EditingForbidden);
    }

    let base_secondary_author = updated_secondary_author_data.map_err(|e| ValidationError {
        reason: e.to_string(),
    })?;

    base_secondary_author
        .validate()
        .map_err(|e| ValidationError {
            reason: e.to_string(),
        })?;

    author_service
        .update_secondary_author_by_id(&author.id, &From::from(base_secondary_author))
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?;

    Ok(().into())
}
