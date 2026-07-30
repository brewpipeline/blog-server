use blog_server_api_macros::ApiRequest;
use blog_server_services::traits::comment_service::CommentService;
use std::sync::Arc;

#[derive(ApiRequest)]
pub struct DeleteCommentRequestContent {
    #[request(path = "id")]
    pub(super) id: String,
    #[request(extension)]
    pub(super) comment_service: Arc<dyn CommentService>,
}
