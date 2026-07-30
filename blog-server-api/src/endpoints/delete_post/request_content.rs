use blog_server_api_macros::ApiRequest;
use blog_server_services::traits::comment_service::CommentService;
use blog_server_services::traits::post_service::PostService;
use std::sync::Arc;

#[derive(ApiRequest)]
pub struct DeletePostRequestContent {
    #[request(path = "id")]
    pub(super) id: String,
    #[request(extension)]
    pub(super) post_service: Arc<dyn PostService>,
    #[request(extension)]
    pub(super) comment_service: Arc<dyn CommentService>,
}
