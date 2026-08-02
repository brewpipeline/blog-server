use blog_generic::entities::CommonComment;
use blog_server_api_macros::ApiRequest;
use blog_server_services::traits::comment_service::CommentService;
use std::sync::Arc;

#[derive(ApiRequest)]
#[request(failure = super::response_content_failure::CreateCommentContentFailure)]
pub struct CreateCommentRequestContent {
    #[request(data)]
    pub(super) new_comment_data: CommonComment,
    #[request(extension)]
    pub(super) comment_service: Arc<dyn CommentService>,
}
