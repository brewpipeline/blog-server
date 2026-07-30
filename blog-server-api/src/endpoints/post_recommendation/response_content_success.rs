use blog_generic::entities::{Post, PostContainer};
use blog_server_api_macros::ApiSuccess;

#[derive(Debug, Clone, ApiSuccess)]
#[success(found, description = "recommended post found")]
pub struct PostRecommendationResponseContentSuccess {
    pub(super) container: PostContainer,
}

impl From<Post> for PostRecommendationResponseContentSuccess {
    fn from(value: Post) -> Self {
        PostRecommendationResponseContentSuccess {
            container: PostContainer { post: value },
        }
    }
}
