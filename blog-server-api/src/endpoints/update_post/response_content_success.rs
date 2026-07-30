use blog_generic::entities::{Post, PostContainer};
use blog_server_api_macros::ApiSuccess;

#[derive(Debug, Clone, ApiSuccess)]
#[success(ok, description = "post record updated")]
pub struct UpdatePostContentSuccess {
    container: PostContainer,
}

impl From<Post> for UpdatePostContentSuccess {
    fn from(value: Post) -> Self {
        UpdatePostContentSuccess {
            container: PostContainer { post: value },
        }
    }
}
