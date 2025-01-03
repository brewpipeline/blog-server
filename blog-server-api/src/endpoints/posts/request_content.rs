use crate::{extensions::Resolve, utils::auth};
use blog_server_services::traits::{
    author_service::{Author, AuthorService},
    entity_post_service::EntityPostService,
    post_service::PostService,
};
use screw_api::request::{ApiRequestContent, ApiRequestOriginContent};
use screw_components::dyn_fn::DFuture;
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
