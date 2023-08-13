use crate::extensions::Resolve;
use blog_server_services::traits::{
    entity_post_service::EntityPostService, post_service::PostService,
};
use screw_api::request::{ApiRequestContent, ApiRequestOriginContent};
use std::sync::Arc;

pub struct PostsRequestContent {
    pub(super) query: Option<String>,
    pub(super) offset: Option<u64>,
    pub(super) limit: Option<u64>,
    pub(super) post_service: Arc<Box<dyn PostService>>,
    pub(super) entity_post_service: Arc<Box<dyn EntityPostService>>,
}

impl<Extensions> ApiRequestContent<Extensions> for PostsRequestContent
where
    Extensions: Resolve<Arc<Box<dyn PostService>>> + Resolve<Arc<Box<dyn EntityPostService>>>,
{
    type Data = ();

    fn create(origin_content: ApiRequestOriginContent<Self::Data, Extensions>) -> Self {
        Self {
            query: origin_content.path.get("query").map(|n| n.to_owned()),
            offset: origin_content
                .query
                .get("offset")
                .map(|v| v.parse().ok())
                .flatten(),
            limit: origin_content
                .query
                .get("limit")
                .map(|v| v.parse().ok())
                .flatten(),
            post_service: origin_content.extensions.resolve(),
            entity_post_service: origin_content.extensions.resolve(),
        }
    }
}
