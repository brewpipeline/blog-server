use blog_server_services::traits::post_service::Post as ServicePost;
use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentSuccess};
use serde::Serialize;

use crate::entities::Post;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePostContentSuccess {
    updated_post: Post,
}

impl Into<UpdatePostContentSuccess> for ServicePost {
    fn into(self) -> UpdatePostContentSuccess {
        UpdatePostContentSuccess {
            updated_post: (self.into()),
        }
    }
}

impl ApiResponseContentBase for UpdatePostContentSuccess {
    fn status_code(&self) -> &'static StatusCode {
        &StatusCode::OK
    }
}

impl ApiResponseContentSuccess for UpdatePostContentSuccess {
    type Data = Self;

    fn identifier(&self) -> &'static str {
        "POST_UPDATED"
    }

    fn description(&self) -> Option<String> {
        Some(String::from("post record updated"))
    }

    fn data(&self) -> &Self::Data {
        self
    }
}
