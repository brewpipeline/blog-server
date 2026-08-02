use std::sync::Arc;

use blog_generic::entities::AuthorsContainer;

use crate::utils::pagination::Pagination;
use blog_server_services::traits::author_service::AuthorService;

use super::request_content::AuthorsRequestContent;
use super::response_content_failure::AuthorsResponseContentFailure;
use super::response_content_success::AuthorsResponseContentSuccess;

pub async fn http_handler(
    (AuthorsRequestContent {
        query,
        offset,
        limit,
        author_service,
    },): (AuthorsRequestContent,),
) -> Result<AuthorsResponseContentSuccess, AuthorsResponseContentFailure> {
    let pagination = Pagination::new(offset, limit, 50);

    let (authors_result, total_result) = if let Some(query) = query {
        tokio::join!(
            author_service.authors_by_query(&query, &pagination.offset, &pagination.limit),
            author_service.authors_count_by_query(&query),
        )
    } else {
        tokio::join!(
            author_service.authors(&pagination.offset, &pagination.limit),
            author_service.authors_count(),
        )
    };

    let authors = authors_result?.into_iter().map(|a| a.into()).collect();

    let total = total_result?;

    Ok(AuthorsContainer {
        authors,
        base: pagination.with_total(total),
    }
    .into())
}

#[cfg_attr(not(feature = "ssr"), allow(dead_code))]
pub async fn direct_handler(
    offset: u64,
    limit: u64,
    author_service: Arc<dyn AuthorService>,
) -> Option<AuthorsContainer> {
    http_handler((AuthorsRequestContent {
        query: None,
        offset: Some(offset),
        limit: Some(limit),
        author_service,
    },))
    .await
    .ok()
    .map(|s| s.container)
}
