use crate::extensions::Resolve;
use blog_server_services::traits::post_service::*;
use screw_api::request::{ApiRequestContent, ApiRequestOriginContent};
use std::sync::Arc;

pub struct PostsRequestContent {
    pub(super) query: Option<String>,
    pub(super) offset: Option<i64>,
    pub(super) limit: Option<i64>,
    pub(super) post_service: Arc<Box<dyn PostService>>,
}

impl<Extensions> ApiRequestContent<Extensions> for PostsRequestContent
where
    Extensions: Resolve<Arc<Box<dyn PostService>>>,
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
        }
    }
}
