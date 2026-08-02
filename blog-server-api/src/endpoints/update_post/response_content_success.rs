use blog_generic::entities::{Post, PostContainer};
use blog_server_api_macros::ApiSuccess;

#[derive(Debug, Clone, ApiSuccess)]
#[success(ok, from = Post, description = "post record updated")]
pub struct UpdatePostContentSuccess {
    container: PostContainer,
}
