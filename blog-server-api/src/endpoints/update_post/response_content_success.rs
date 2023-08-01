use blog_generic::entities::PostContainer;
use blog_server_services::traits::post_service::Post as ServicePost;
use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentSuccess};

#[derive(Debug, Clone)]
pub struct UpdatePostContentSuccess {
    container: PostContainer,
}

impl Into<UpdatePostContentSuccess> for ServicePost {
    fn into(self) -> UpdatePostContentSuccess {
        UpdatePostContentSuccess {
            container: PostContainer { post: self.into() },
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
