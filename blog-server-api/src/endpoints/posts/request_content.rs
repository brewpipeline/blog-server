use crate::extensions::Resolve;
use blog_server_services::traits::{
    entity_post_service::EntityPostService, post_service::PostService,
};
use screw_api::request::{ApiRequestContent, ApiRequestOriginContent};
use screw_api::response::ApiResponseContentFailure;
use std::sync::Arc;

pub struct PostsRequestContentFilter {
    pub search_query: Option<String>,
    pub author_id: Option<u64>,
    pub tag_id: Option<u64>,
}

pub struct PostsRequestContent {
    pub(super) filter: PostsRequestContentFilter,
    pub(super) offset: Option<u64>,
    pub(super) limit: Option<u64>,
    pub(super) post_service: Arc<dyn PostService>,
    pub(super) entity_post_service: Arc<dyn EntityPostService>,
}

impl<Extensions, Failure> ApiRequestContent<Extensions, Failure> for PostsRequestContent
where
    Extensions: Resolve<Arc<dyn PostService>> + Resolve<Arc<dyn EntityPostService>>,
    Failure: ApiResponseContentFailure,
{
    type Data = ();

    fn create(
        origin_content: ApiRequestOriginContent<Self::Data, Extensions>,
    ) -> Result<Self, Failure> {
        Ok(Self {
            filter: PostsRequestContentFilter {
                search_query: origin_content
                    .query
                    .get("search_query")
                    .map(|n| n.to_owned()),
                author_id: origin_content
                    .query
                    .get("author_id")
                    .map(|n| n.parse().ok())
                    .flatten(),
                tag_id: origin_content
                    .query
                    .get("tag_id")
                    .map(|n| n.parse().ok())
                    .flatten(),
            },
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
        })
    }
}
