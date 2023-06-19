use super::request_content::MeRequestContent;
use super::response_content_failure::MeResponseContentFailure;
use super::response_content_failure::MeResponseContentFailure::*;
use super::response_content_success::MeResponseContentSuccess;
use crate::extensions::Resolve;
use crate::utils::login;
use blog_server_services::traits::author_service::{Author, AuthorService};
use screw_api::request::ApiRequest;
use screw_api::response::ApiResponse;
use screw_components::dyn_fn::DFuture;
use std::sync::Arc;

async fn handler(
    self_author_fut: DFuture<Result<Author, login::Error>>,
) -> Result<MeResponseContentSuccess, MeResponseContentFailure> {
    self_author_fut
        .await
        .map(|v| v.into())
        .map_err(|e| Unauthorized {
            reason: e.to_string(),
        })
}

pub async fn http_handler<Extensions>(
    request: ApiRequest<MeRequestContent, Extensions>,
) -> ApiResponse<MeResponseContentSuccess, MeResponseContentFailure>
where
    Extensions: Resolve<Arc<Box<dyn AuthorService>>>,
{
    ApiResponse::from(handler(request.content.self_author_fut).await)
}
