use blog_server_services::traits::author_service::Author;
use validator::Validate;

use super::request_content::UpdateMinimalAuthorRequestContent;
use super::response_content_failure::UpdateMinimalAuthorContentFailure;
use super::response_content_failure::UpdateMinimalAuthorContentFailure::*;
use super::response_content_success::UpdateMinimalAuthorContentSuccess;

pub async fn http_handler(
    (
        author,
        UpdateMinimalAuthorRequestContent {
            updated_minimal_author_data: base_minimal_author,
            author_service,
        },
    ): (Author, UpdateMinimalAuthorRequestContent),
) -> Result<UpdateMinimalAuthorContentSuccess, UpdateMinimalAuthorContentFailure> {
    base_minimal_author
        .validate()
        .map_err(|e| ValidationError {
            reason: e.to_string(),
        })?;

    author_service
        .update_minimal_custom_author_by_id(&author.id, &From::from(base_minimal_author))
        .await?;

    Ok(().into())
}
