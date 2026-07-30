use blog_server_api_macros::ApiRequest;
use blog_server_services::traits::author_service::*;
use std::sync::Arc;

#[derive(ApiRequest)]
pub struct AuthorsRequestContent {
    #[request(path = "query")]
    pub(super) query: Option<String>,
    #[request(query = "offset")]
    pub(super) offset: Option<u64>,
    #[request(query = "limit")]
    pub(super) limit: Option<u64>,
    #[request(extension)]
    pub(super) author_service: Arc<dyn AuthorService>,
}
