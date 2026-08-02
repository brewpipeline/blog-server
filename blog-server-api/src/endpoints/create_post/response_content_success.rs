use blog_generic::entities::{Post, PostContainer};
use blog_server_api_macros::ApiSuccess;

#[derive(Debug, Clone, ApiSuccess)]
#[success(created, from = Post, description = "post record created")]
pub struct CreatePostContentSuccess {
    container: PostContainer,
}
