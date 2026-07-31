use blog_server_api_macros::ApiRequest;
use blog_server_services::traits::comment_service::*;
use blog_server_services::traits::entity_comment_service::*;
use std::sync::Arc;

#[derive(ApiRequest)]
#[request(failure = super::response_content_failure::CommentsResponseContentFailure)]
pub struct CommentsRequestContent {
    #[request(path = "post_id")]
    pub(super) post_id: u64,
    #[request(query = "offset")]
    pub(super) offset: Option<u64>,
    #[request(query = "limit")]
    pub(super) limit: Option<u64>,
    #[request(extension)]
    pub(super) comment_service: Arc<dyn CommentService>,
    #[request(extension)]
    pub(super) entity_comment_service: Arc<dyn EntityCommentService>,
}
