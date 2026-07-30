use blog_server_api_macros::ApiRequest;
use blog_server_services::traits::post_service::PostService;
use std::sync::Arc;

#[derive(ApiRequest)]
pub struct TagRequestContent {
    #[request(path = "id")]
    pub(super) id: String,
    #[request(extension)]
    pub(super) post_service: Arc<dyn PostService>,
}
