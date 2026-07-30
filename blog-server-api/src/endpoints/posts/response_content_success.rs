use blog_generic::entities::PostsContainer;
use blog_server_api_macros::ApiSuccess;

#[derive(Debug, Clone, ApiSuccess)]
#[success(found, description = "posts list returned")]
pub struct PostsResponseContentSuccess {
    pub(super) container: PostsContainer,
}

impl From<PostsContainer> for PostsResponseContentSuccess {
    fn from(value: PostsContainer) -> Self {
        PostsResponseContentSuccess { container: value }
    }
}
