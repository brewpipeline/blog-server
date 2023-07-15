use crate::{entities::CreatePost, extensions::Resolve};
use blog_server_services::traits::post_service::PostService;
use screw_api::request::{ApiRequestContent, ApiRequestOriginContent};
use screw_components::dyn_result::DResult;
use std::sync::Arc;

pub struct CreatePostRequestContent {
    pub(super) new_post_data: DResult<CreatePost>,
    pub(super) post_service: Arc<Box<dyn PostService>>,
}

impl<Extensions> ApiRequestContent<Extensions> for CreatePostRequestContent
where
    Extensions: Resolve<Arc<Box<dyn PostService>>>,
{
    type Data = CreatePost;

    fn create(origin_content: ApiRequestOriginContent<Self::Data, Extensions>) -> Self {
        Self {
            new_post_data: origin_content.data_result,
            post_service: origin_content.extensions.resolve(),
        }
    }
}
