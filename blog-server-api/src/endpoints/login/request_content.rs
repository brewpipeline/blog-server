use crate::extensions::Resolve;
use blog_generic::entities::LoginQuestion;
use blog_server_services::traits::author_service::*;
use screw_api::request::{ApiRequestContent, ApiRequestOriginContent};
use screw_components::dyn_result::DResult;
use std::sync::Arc;

pub struct LoginRequestContent {
    pub(super) login_question: DResult<LoginQuestion>,
    pub(super) author_service: Arc<dyn AuthorService>,
}

impl<Extensions> ApiRequestContent<Extensions> for LoginRequestContent
where
    Extensions: Resolve<Arc<dyn AuthorService>>,
{
    type Data = LoginQuestion;

    fn create(origin_content: ApiRequestOriginContent<Self::Data, Extensions>) -> Self {
        Self {
            login_question: origin_content.data_result,
            author_service: origin_content.extensions.resolve(),
        }
    }
}
