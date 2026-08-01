use blog_generic::entities::CommentsContainer;
use blog_server_api_macros::ApiSuccess;

#[derive(Debug, Clone, ApiSuccess)]
#[success(found, description = "comments list returned")]
pub struct CommentsResponseContentSuccess {
    pub(super) container: CommentsContainer,
}
