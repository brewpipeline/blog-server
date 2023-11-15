use crate::extensions::Resolve;
use blog_generic::entities::LoginTelegramQuestion;
use blog_server_services::traits::author_service::*;
use screw_api::request::{ApiRequestContent, ApiRequestOriginContent};
use screw_components::dyn_result::DResult;
use std::sync::Arc;

pub struct LoginTelegramRequestContent {
    pub(super) login_telegram_question: DResult<LoginTelegramQuestion>,
    pub(super) author_service: Arc<Box<dyn AuthorService>>,
}

impl<Extensions> ApiRequestContent<Extensions> for LoginTelegramRequestContent
where
    Extensions: Resolve<Arc<Box<dyn AuthorService>>>,
{
    type Data = LoginTelegramQuestion;

    fn create(origin_content: ApiRequestOriginContent<Self::Data, Extensions>) -> Self {
        Self {
            login_telegram_question: origin_content.data_result,
            author_service: origin_content.extensions.resolve(),
        }
    }
}
