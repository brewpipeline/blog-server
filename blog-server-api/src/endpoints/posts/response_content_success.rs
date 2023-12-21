use blog_generic::entities::PostsContainer;
use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentSuccess};

#[derive(Debug, Clone)]
pub struct PostsResponseContentSuccess {
    pub container: PostsContainer,
}

impl Into<PostsResponseContentSuccess> for PostsContainer {
    fn into(self) -> PostsResponseContentSuccess {
        PostsResponseContentSuccess { container: self }
    }
}

impl ApiResponseContentBase for PostsResponseContentSuccess {
    fn status_code(&self) -> &'static StatusCode {
        &StatusCode::OK
    }
}

impl ApiResponseContentSuccess for PostsResponseContentSuccess {
    type Data = PostsContainer;

    fn identifier(&self) -> &'static str {
        "POSTS_OK"
    }

    fn description(&self) -> Option<String> {
        Some("posts list returned".to_string())
    }

    fn data(&self) -> &Self::Data {
        &self.container
    }
}
