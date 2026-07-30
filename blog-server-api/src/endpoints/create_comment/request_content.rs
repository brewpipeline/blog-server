use blog_generic::entities::CommonComment;
use blog_server_api_macros::ApiRequest;
use blog_server_services::traits::comment_service::CommentService;
use screw_components::dyn_result::DResult;
use std::sync::Arc;

#[derive(ApiRequest)]
pub struct CreateCommentRequestContent {
    #[request(data)]
    pub(super) new_comment_data: DResult<CommonComment>,
    #[request(extension)]
    pub(super) comment_service: Arc<dyn CommentService>,
}
