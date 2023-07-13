use crate::extensions::Resolve;
use blog_server_services::traits::post_service::*;
use screw_api::request::{ApiRequestContent, ApiRequestOriginContent};
use screw_components::dyn_result::DResult;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePostRequestData {
    pub post_id: i64,
    pub post_content: String,
}

pub struct CreatePostRequestContent {
    pub(super) new_post_data: DResult<CreatePostRequestData>,
    pub(super) post_service: Arc<Box<dyn PostService>>,
}

impl<Extensions> ApiRequestContent<Extensions> for CreatePostRequestContent
where
    Extensions: Resolve<Arc<Box<dyn PostService>>>,
{
    type Data = CreatePostRequestData;

    fn create(origin_content: ApiRequestOriginContent<Self::Data, Extensions>) -> Self {
        Self {
            new_post_data: origin_content.data_result,
            post_service: origin_content.extensions.resolve(),
        }
    }
}
