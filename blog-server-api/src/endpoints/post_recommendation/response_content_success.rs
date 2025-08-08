use blog_generic::entities::{Post, PostContainer};
use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentSuccess};

#[derive(Debug, Clone)]
pub struct PostRecommendationResponseContentSuccess {
    pub(super) container: PostContainer,
}

impl Into<PostRecommendationResponseContentSuccess> for Post {
    fn into(self) -> PostRecommendationResponseContentSuccess {
        PostRecommendationResponseContentSuccess {
            container: PostContainer { post: self },
        }
    }
}

impl ApiResponseContentBase for PostRecommendationResponseContentSuccess {
    fn status_code(&self) -> &'static StatusCode {
        &StatusCode::OK
    }
}

impl ApiResponseContentSuccess for PostRecommendationResponseContentSuccess {
    type Data = PostContainer;

    fn identifier(&self) -> &'static str {
        "POST_RECOMMENDATION_FOUND"
    }

    fn description(&self) -> Option<String> {
        Some("recommended post found".to_string())
    }

    fn data(&self) -> &Self::Data {
        &self.container
    }
}
