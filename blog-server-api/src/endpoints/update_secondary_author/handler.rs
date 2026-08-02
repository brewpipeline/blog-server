use blog_server_services::traits::author_service::Author;
use validator::Validate;

use super::request_content::UpdateSecondaryAuthorRequestContent;
use super::response_content_failure::UpdateSecondaryAuthorContentFailure;
use super::response_content_failure::UpdateSecondaryAuthorContentFailure::*;
use super::response_content_success::UpdateSecondaryAuthorContentSuccess;

pub async fn http_handler(
    (
        author,
        UpdateSecondaryAuthorRequestContent {
            updated_secondary_author_data: base_secondary_author,
            author_service,
        },
    ): (Author, UpdateSecondaryAuthorRequestContent),
) -> Result<UpdateSecondaryAuthorContentSuccess, UpdateSecondaryAuthorContentFailure> {
    base_secondary_author
        .validate()
        .map_err(|e| ValidationError {
            reason: e.to_string(),
        })?;

    author_service
        .update_secondary_author_by_id(&author.id, &From::from(base_secondary_author))
        .await?;

    Ok(UpdateSecondaryAuthorContentSuccess)
}
