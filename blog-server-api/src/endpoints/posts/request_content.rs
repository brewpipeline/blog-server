use crate::{extensions::Resolve, utils::auth};
use blog_server_services::traits::{
    author_service::{Author, AuthorService},
    entity_post_service::EntityPostService,
    post_service::PostService,
};
use screw_api::request::{ApiRequestContent, ApiRequestOriginContent};
use screw_components::dyn_fn::DFuture;
use std::sync::Arc;

pub enum PostsRequestContentFilter {
    SearchQuery(String),
    AuthorId(u64),
    TagId(u64),
}

pub struct PostsRequestContent {
    pub(crate) filter: Option<PostsRequestContentFilter>,
    pub(crate) offset: Option<u64>,
    pub(crate) limit: Option<u64>,
    pub(crate) post_service: Arc<Box<dyn PostService>>,
    pub(crate) entity_post_service: Arc<Box<dyn EntityPostService>>,
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

pub struct UnpublishedPostsRequestContent {
    pub(super) base: PostsRequestContent,
    pub(super) auth_author_future: DFuture<Result<Author, auth::Error>>,
}

impl<Extensions> ApiRequestContent<Extensions> for UnpublishedPostsRequestContent
where
    Extensions: Resolve<Arc<Box<dyn PostService>>>
        + Resolve<Arc<Box<dyn EntityPostService>>>
        + Resolve<Arc<Box<dyn AuthorService>>>,
{
    type Data = ();

    fn create(origin_content: ApiRequestOriginContent<Self::Data, Extensions>) -> Self {
        let auth_author_future = Box::pin(auth::author(
            &origin_content.http_parts,
            origin_content.extensions.resolve(),
        ));
        Self {
            base: PostsRequestContent::create(origin_content),
            auth_author_future,
        }
    }
}
