use blog_server_api_macros::ApiRequest;
use blog_server_services::traits::comment_service::CommentService;
use std::sync::Arc;

#[derive(ApiRequest)]
#[request(failure = super::response_content_failure::DeleteCommentResponseContentFailure)]
pub struct DeleteCommentRequestContent {
    #[request(path = "id")]
    pub(super) id: u64,
    #[request(extension)]
    pub(super) comment_service: Arc<dyn CommentService>,
}
