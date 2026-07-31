use blog_server_api_macros::ApiRequest;
use blog_server_services::traits::author_service::*;
use std::sync::Arc;

#[derive(ApiRequest)]
pub struct AuthorBlockRequestContent {
    #[request(path = "id")]
    pub(super) id: Result<u64, std::num::ParseIntError>,
    #[request(extension)]
    pub(super) author_service: Arc<dyn AuthorService>,
}
