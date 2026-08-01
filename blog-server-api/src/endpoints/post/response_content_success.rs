use blog_generic::entities::{Post, PostContainer};
use blog_server_api_macros::ApiSuccess;

#[derive(Debug, Clone, ApiSuccess)]
#[success(found, from = Post, description = "post record found")]
pub struct PostResponseContentSuccess {
    pub(super) container: PostContainer,
}
