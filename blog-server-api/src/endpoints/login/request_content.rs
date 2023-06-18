use crate::extensions::Resolve;
use blog_server_services::traits::author_service::*;
use screw_api::request::{ApiRequestContent, ApiRequestOriginContent};
use screw_components::dyn_result::DResult;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct LoginRequestContentData {
    pub authorname: String,
    pub password: String,
}

pub struct LoginRequestContent {
    pub(super) login_data: DResult<LoginRequestContentData>,
    pub(super) author_service: Arc<Box<dyn AuthorService>>,
}

impl<Extensions> ApiRequestContent<Extensions> for LoginRequestContent
where
    Extensions: Resolve<Arc<Box<dyn AuthorService>>>,
{
    type Data = LoginRequestContentData;

    fn create(origin_content: ApiRequestOriginContent<Self::Data, Extensions>) -> Self {
        Self {
            login_data: origin_content.data_result,
            author_service: origin_content.extensions.resolve(),
        }
    }
}
