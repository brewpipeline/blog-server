use blog_generic::entities::{Post, PostContainer};
use blog_server_api_macros::ApiSuccess;

#[derive(Debug, Clone, ApiSuccess)]
#[success(created, description = "post record created")]
pub struct CreatePostContentSuccess {
    container: PostContainer,
}

impl From<Post> for CreatePostContentSuccess {
    fn from(value: Post) -> Self {
        CreatePostContentSuccess {
            container: PostContainer { post: value },
        }
    }
}
