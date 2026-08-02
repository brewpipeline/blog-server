use blog_server_api_macros::ApiRequest;
use blog_server_services::traits::author_service::*;
use std::sync::Arc;

#[derive(ApiRequest)]
#[request(failure = super::response_content_failure::AuthorBlockResponseContentFailure)]
pub struct AuthorBlockRequestContent {
    #[request(path = "id")]
    pub(super) id: u64,
    #[request(extension)]
    pub(super) author_service: Arc<dyn AuthorService>,
}
