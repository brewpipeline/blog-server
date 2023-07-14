use super::request_content::AuthorRequestContent;
use super::response_content_failure::AuthorResponseContentFailure;
use super::response_content_failure::AuthorResponseContentFailure::*;
use super::response_content_success::AuthorResponseContentSuccess;

pub async fn http_handler(
    (AuthorRequestContent {
        slug,
        author_service,
    },): (AuthorRequestContent,),
) -> Result<AuthorResponseContentSuccess, AuthorResponseContentFailure> {
    if slug.is_empty() {
        return Err(SlugEmpty);
    }

    let author = author_service
        .author_by_slug(&slug)
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?
        .ok_or(NotFound)?;

    Ok(author.into())
}
