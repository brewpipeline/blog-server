use crate::{extensions::Resolve, utils::auth};
use blog_generic::entities::CommonComment;
use blog_server_services::traits::{
    author_service::{Author, AuthorService},
    comment_service::CommentService,
};
use screw_api::request::{ApiRequestContent, ApiRequestOriginContent};
use screw_components::{dyn_fn::DFuture, dyn_result::DResult};
use std::sync::Arc;

pub struct CreateCommentRequestContent {
    pub(super) new_comment_data: DResult<CommonComment>,
    pub(super) comment_service: Arc<Box<dyn CommentService>>,
    pub(super) auth_author_future: DFuture<Result<Author, auth::Error>>,
}

impl<Extensions> ApiRequestContent<Extensions> for CreateCommentRequestContent
where
    Extensions: Resolve<Arc<Box<dyn CommentService>>> + Resolve<Arc<Box<dyn AuthorService>>>,
{
    type Data = CommonComment;

    fn create(origin_content: ApiRequestOriginContent<Self::Data, Extensions>) -> Self {
        Self {
            new_comment_data: origin_content.data_result,
            comment_service: origin_content.extensions.resolve(),
            auth_author_future: Box::pin(auth::author(
                &origin_content.http_parts,
                origin_content.extensions.resolve(),
            )),
        }
    }
}
