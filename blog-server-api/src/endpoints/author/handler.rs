use super::request_content::AuthorRequestContent;
use super::response_content_failure::AuthorResponseContentFailure;
use super::response_content_failure::AuthorResponseContentFailure::*;
use super::response_content_success::AuthorResponseContentSuccess;
use crate::extensions::Resolve;
use blog_server_services::traits::author_service::AuthorService;
use screw_api::request::ApiRequest;
use screw_api::response::ApiResponse;
use std::sync::Arc;

async fn handler(
    authorname: String,
    author_service: Arc<Box<dyn AuthorService>>,
) -> Result<AuthorResponseContentSuccess, AuthorResponseContentFailure> {
    if authorname.is_empty() {
        return Err(NameEmpty);
    }

    let author = author_service
        .get_author(&authorname)
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?
        .ok_or(NotFound)?;

    Ok(author.into())
}

pub async fn http_handler<Extensions>(
    request: ApiRequest<AuthorRequestContent, Extensions>,
) -> ApiResponse<AuthorResponseContentSuccess, AuthorResponseContentFailure>
where
    Extensions: Resolve<Arc<Box<dyn AuthorService>>>,
{
    ApiResponse::from(handler(request.content.authorname, request.content.author_service).await)
}
