use crate::extensions::Resolve;
use blog_server_services::traits::{
    entity_post_service::EntityPostService, post_service::PostService,
};
use screw_api::request::{ApiRequestContent, ApiRequestOriginContent};
use std::sync::Arc;

pub enum PostsRequestContentFilter {
    SearchQuery(String),
    AuthorId(u64),
    TagId(u64),
}

pub struct PostsRequestContent {
    pub(super) filter: Option<PostsRequestContentFilter>,
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
            filter: {
                if let Some(search_query) = origin_content
                    .path
                    .get("search_query")
                    .map(|n| n.to_owned())
                {
                    Some(PostsRequestContentFilter::SearchQuery(search_query))
                } else if let Some(author_id) = origin_content
                    .path
                    .get("author_id")
                    .map(|n| n.parse().ok())
                    .flatten()
                {
                    Some(PostsRequestContentFilter::AuthorId(author_id))
                } else if let Some(tag_id) = origin_content
                    .path
                    .get("tag_id")
                    .map(|n| n.parse().ok())
                    .flatten()
                {
                    Some(PostsRequestContentFilter::TagId(tag_id))
                } else {
                    None
                }
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
        }
    }
}
