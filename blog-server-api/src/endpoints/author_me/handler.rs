use super::request_content::AuthorMeRequestContent;
use super::response_content_failure::AuthorMeResponseContentFailure;
use super::response_content_failure::AuthorMeResponseContentFailure::*;
use super::response_content_success::AuthorMeResponseContentSuccess;
use crate::extensions::Resolve;
use crate::utils::login;
use blog_server_services::traits::author_service::{Author, AuthorService};
use screw_api::request::ApiRequest;
use screw_api::response::ApiResponse;
use screw_components::dyn_fn::DFuture;
use std::sync::Arc;

async fn handler(
    auth_author_future: DFuture<Result<Author, login::Error>>,
) -> Result<AuthorMeResponseContentSuccess, AuthorMeResponseContentFailure> {
    auth_author_future
        .await
        .map(|v| v.into())
        .map_err(|e| Unauthorized {
            reason: e.to_string(),
        })
}

pub async fn http_handler<Extensions>(
    request: ApiRequest<AuthorMeRequestContent, Extensions>,
) -> ApiResponse<AuthorMeResponseContentSuccess, AuthorMeResponseContentFailure>
where
    Extensions: Resolve<Arc<Box<dyn AuthorService>>>,
{
    ApiResponse::from(handler(request.content.auth_author_future).await)
}
