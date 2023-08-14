use blog_generic::entities::{Post, PostContainer};
use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentSuccess};

#[derive(Debug, Clone)]
pub struct UpdatePostContentSuccess {
    container: PostContainer,
}

impl Into<UpdatePostContentSuccess> for Post {
    fn into(self) -> UpdatePostContentSuccess {
        UpdatePostContentSuccess {
            container: PostContainer { post: self },
        }
    }
}

impl ApiResponseContentBase for UpdatePostContentSuccess {
    fn status_code(&self) -> &'static StatusCode {
        &StatusCode::OK
    }
}

impl ApiResponseContentSuccess for UpdatePostContentSuccess {
    type Data = PostContainer;

    fn identifier(&self) -> &'static str {
        "UPDATE_POST_SUCCESS"
    }

    fn description(&self) -> Option<String> {
        Some(String::from("post record updated"))
    }

    fn data(&self) -> &Self::Data {
        &self.container
    }
}
