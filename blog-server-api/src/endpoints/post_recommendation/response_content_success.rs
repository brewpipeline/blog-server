use blog_generic::entities::{Post, PostContainer};
use blog_server_api_macros::ApiSuccess;

#[derive(Debug, Clone, ApiSuccess)]
#[success(found, from = Post, description = "recommended post found")]
pub struct PostRecommendationResponseContentSuccess {
    pub(super) container: PostContainer,
}
