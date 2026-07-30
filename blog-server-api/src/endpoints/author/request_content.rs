use blog_server_api_macros::ApiRequest;
use blog_server_services::traits::author_service::*;
use std::sync::Arc;

#[derive(ApiRequest)]
pub struct AuthorRequestContent {
    #[request(path = "slug")]
    pub(super) slug: String,
    #[request(extension)]
    pub(super) author_service: Arc<dyn AuthorService>,
}
