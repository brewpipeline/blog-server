use std::sync::Arc;

use blog_generic::entities::{AuthorsContainer, TotalOffsetLimitContainer};
use blog_server_services::traits::author_service::AuthorService;

use super::request_content::AuthorsRequestContent;
use super::response_content_failure::AuthorsResponseContentFailure;
use super::response_content_failure::AuthorsResponseContentFailure::*;
use super::response_content_success::AuthorsResponseContentSuccess;

pub async fn http_handler(
    (AuthorsRequestContent {
        query,
        offset,
        limit,
        author_service,
    },): (AuthorsRequestContent,),
) -> Result<AuthorsResponseContentSuccess, AuthorsResponseContentFailure> {
    let offset = offset.unwrap_or(0).max(0);
    let limit = limit.unwrap_or(50).max(0).min(50);

    let (authors_result, total_result) = if let Some(query) = query {
        tokio::join!(
            author_service.authors_by_query(&query, &offset, &limit),
            author_service.authors_count_by_query(&query),
        )
    } else {
        tokio::join!(
            author_service.authors(&offset, &limit),
            author_service.authors_count(),
        )
    };

    let authors = authors_result
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?
        .into_iter()
        .map(|a| a.into())
        .collect();

    let total = total_result.map_err(|e| DatabaseError {
        reason: e.to_string(),
    })?;

    Ok(AuthorsContainer {
        authors,
        base: TotalOffsetLimitContainer {
            total,
            offset,
            limit,
        },
    }
    .into())
}

pub async fn direct_handler(
    offset: u64,
    limit: u64,
    author_service: Arc<Box<dyn AuthorService>>,
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
