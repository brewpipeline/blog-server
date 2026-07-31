use blog_server_api_macros::ApiRequest;
use blog_server_services::traits::post_service::PostService;
use std::sync::Arc;

#[derive(ApiRequest)]
#[request(failure = super::response_content_failure::TagResponseContentFailure)]
pub struct TagRequestContent {
    #[request(path = "id")]
    pub(super) id: u64,
    #[request(extension)]
    pub(super) post_service: Arc<dyn PostService>,
}
