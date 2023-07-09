use super::request_content::AuthorsRequestContent;
use super::response_content_failure::AuthorsResponseContentFailure;
use super::response_content_failure::AuthorsResponseContentFailure::*;
use super::response_content_success::AuthorsResponseContentSuccess;
use crate::extensions::Resolve;
use blog_server_services::traits::author_service::AuthorService;
use screw_api::request::ApiRequest;
use screw_api::response::ApiResponse;
use std::sync::Arc;

async fn handler(
    offset: Option<i64>,
    limit: Option<i64>,
    author_service: Arc<Box<dyn AuthorService>>,
) -> Result<AuthorsResponseContentSuccess, AuthorsResponseContentFailure> {
    let offset = offset.unwrap_or(0).max(0);
    let limit = limit.unwrap_or(50).max(0).min(50);

    let (authors_result, total_result) = tokio::join!(
        author_service.authors(&offset, &limit),
        author_service.authors_count(),
    );

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

    Ok(AuthorsResponseContentSuccess {
        authors,
        total,
        offset,
        limit,
    })
}

pub async fn http_handler<Extensions>(
    request: ApiRequest<AuthorsRequestContent, Extensions>,
) -> ApiResponse<AuthorsResponseContentSuccess, AuthorsResponseContentFailure>
where
    Extensions: Resolve<Arc<Box<dyn AuthorService>>>,
{
    ApiResponse::from(
        handler(
            request.content.offset,
            request.content.limit,
            request.content.author_service,
        )
        .await,
    )
}
