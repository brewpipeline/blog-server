use crate::extensions::Resolve;
use blog_server_services::traits::comment_service::*;
use blog_server_services::traits::entity_comment_service::*;
use screw_api::request::{ApiRequestContent, ApiRequestOriginContent};
use std::sync::Arc;

pub struct CommentsRequestContent {
    pub(super) post_id: String,
    pub(super) offset: Option<u64>,
    pub(super) limit: Option<u64>,
    pub(super) comment_service: Arc<dyn CommentService>,
    pub(super) entity_comment_service: Arc<dyn EntityCommentService>,
}

impl<Extensions> ApiRequestContent<Extensions> for CommentsRequestContent
where
    Extensions: Resolve<Arc<dyn CommentService>> + Resolve<Arc<dyn EntityCommentService>>,
{
    type Data = ();

    fn create(origin_content: ApiRequestOriginContent<Self::Data, Extensions>) -> Self {
        Self {
            post_id: origin_content
                .path
                .get("post_id")
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
            entity_comment_service: origin_content.extensions.resolve(),
        }
    }
}
