use std::sync::Arc;

use blog_generic::entities::AuthorContainer;
use blog_server_services::traits::author_service::AuthorService;

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

pub async fn direct_handler(
    slug: String,
    author_service: Arc<Box<dyn AuthorService>>,
) -> Option<AuthorContainer> {
    http_handler((AuthorRequestContent {
        slug,
        author_service,
    },))
    .await
    .ok()
    .map(|s| s.container)
}
