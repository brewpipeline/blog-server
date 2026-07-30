use blog_generic::entities::{Post, PostContainer};
use blog_server_api_macros::ApiSuccess;

#[derive(Debug, Clone, ApiSuccess)]
#[success(found, description = "post record found")]
pub struct PostResponseContentSuccess {
    pub(super) container: PostContainer,
}

impl From<Post> for PostResponseContentSuccess {
    fn from(value: Post) -> Self {
        PostResponseContentSuccess {
            container: PostContainer { post: value },
        }
    }
}
