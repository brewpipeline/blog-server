use crate::extensions::Resolve;
use blog_server_services::traits::comment_service::*;
use blog_server_services::traits::post_service::*;
use screw_api::request::{ApiRequestContent, ApiRequestOriginContent};
use std::sync::Arc;

pub struct CommentsRequestContent {
    pub(super) post_slug: String,
    pub(super) offset: Option<i64>,
    pub(super) limit: Option<i64>,
    pub(super) comment_service: Arc<Box<dyn CommentService>>,
    pub(super) post_service: Arc<Box<dyn PostService>>,
}

impl<Extensions> ApiRequestContent<Extensions> for CommentsRequestContent
where
    Extensions: Resolve<Arc<Box<dyn CommentService>>> + Resolve<Arc<Box<dyn PostService>>>,
{
    type Data = ();

    fn create(origin_content: ApiRequestOriginContent<Self::Data, Extensions>) -> Self {
        Self {
            post_slug: origin_content
                .path
                .get("post_slug")
                .map(|n| n.to_owned())
                .unwrap_or_default(),
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
            comment_service: origin_content.extensions.resolve(),
            post_service: origin_content.extensions.resolve(),
        }
    }
}
