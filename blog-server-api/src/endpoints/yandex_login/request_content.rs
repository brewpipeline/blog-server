use crate::extensions::Resolve;
use blog_generic::entities::LoginYandexQuestion;
use blog_server_services::traits::author_service::*;
use screw_api::request::{ApiRequestContent, ApiRequestOriginContent};
use screw_components::dyn_result::DResult;
use std::sync::Arc;

pub struct LoginYandexRequestContent {
    pub(super) login_yandex_question: DResult<LoginYandexQuestion>,
    pub(super) author_service: Arc<Box<dyn AuthorService>>,
}

impl<Extensions> ApiRequestContent<Extensions> for LoginYandexRequestContent
where
    Extensions: Resolve<Arc<Box<dyn AuthorService>>>,
{
    type Data = LoginYandexQuestion;

    fn create(origin_content: ApiRequestOriginContent<Self::Data, Extensions>) -> Self {
        Self {
            login_yandex_question: origin_content.data_result,
            author_service: origin_content.extensions.resolve(),
        }
    }
}
