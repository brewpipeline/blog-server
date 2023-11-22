use blog_generic::entities::CommonSecondaryAuthor;
use blog_server_services::traits::author_service::{Author, AuthorService};
use screw_api::request::ApiRequestContent;
use screw_components::{dyn_fn::DFuture, dyn_result::DResult};
use std::sync::Arc;

use crate::{extensions::Resolve, utils::auth};

pub struct UpdateSecondaryAuthorRequestContent {
    pub(super) updated_secondary_author_data: DResult<CommonSecondaryAuthor>,
    pub(super) author_service: Arc<Box<dyn AuthorService>>,
    pub(super) auth_author_future: DFuture<Result<Author, auth::Error>>,
}

impl<Extensions> ApiRequestContent<Extensions> for UpdateSecondaryAuthorRequestContent
where
    Extensions: Resolve<Arc<Box<dyn AuthorService>>>,
{
    type Data = CommonSecondaryAuthor;

    fn create(
        origin_content: screw_api::request::ApiRequestOriginContent<Self::Data, Extensions>,
    ) -> Self {
        Self {
            updated_secondary_author_data: origin_content.data_result,
            author_service: origin_content.extensions.resolve(),
            auth_author_future: Box::pin(auth::author(
                &origin_content.http_parts,
                origin_content.extensions.resolve(),
            )),
        }
    }
}
