use crate::extensions::Resolve;
use blog_server_services::traits::{
    entity_post_service::EntityPostService, post_service::PostService,
};
use screw_api::request::{ApiRequestContent, ApiRequestOriginContent};
use std::sync::Arc;

pub struct PostRequestContent {
    pub(super) id: String,
    pub(super) post_service: Arc<Box<dyn PostService>>,
    pub(super) entity_post_service: Arc<Box<dyn EntityPostService>>,
}

impl<Extensions> ApiRequestContent<Extensions> for PostRequestContent
where
    Extensions: Resolve<Arc<Box<dyn PostService>>> + Resolve<Arc<Box<dyn EntityPostService>>>,
{
    type Data = ();

    fn create(origin_content: ApiRequestOriginContent<Self::Data, Extensions>) -> Self {
        Self {
            id: origin_content
                .path
                .get("id")
                .map(|n| n.to_owned())
                .unwrap_or_default(),
            post_service: origin_content.extensions.resolve(),
            entity_post_service: origin_content.extensions.resolve(),
        }
    }
}
