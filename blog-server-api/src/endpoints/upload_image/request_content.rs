use crate::{extensions::Resolve, utils::auth};
use blog_server_services::traits::author_service::{Author, AuthorService};
use hyper::header;
use screw_api::request::{ApiRequestContent, ApiRequestOriginContent};
use screw_components::dyn_fn::DFuture;
use screw_components::dyn_result::DResult;
use std::sync::Arc;

pub struct UploadImageRequestContent {
    pub(super) upload_data: DResult<Vec<u8>>,
    pub(super) content_type: Option<String>,
    pub(super) auth_author_future: DFuture<Result<Author, auth::Error>>,
}

impl<Extensions> ApiRequestContent<Extensions> for UploadImageRequestContent
where
    Extensions: Resolve<Arc<dyn AuthorService>>,
{
    type Data = Vec<u8>;

    fn create(origin_content: ApiRequestOriginContent<Self::Data, Extensions>) -> Self {
        let content_type = origin_content
            .http_parts
            .headers
            .get(header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());
        Self {
            upload_data: origin_content.data_result,
            content_type,
            auth_author_future: Box::pin(auth::author(
                &origin_content.http_parts,
                origin_content.extensions.resolve(),
            )),
        }
    }
}

