use crate::extensions::Resolve;
use blog_generic::entities::LoginTelegramQuestion;
use blog_server_services::traits::social_service::SocialService;
use screw_api::request::{ApiRequestContent, ApiRequestOriginContent};
use screw_components::dyn_result::DResult;
use std::sync::Arc;

pub struct LoginTelegramRequestContent {
    pub(super) login_telegram_question: DResult<LoginTelegramQuestion>,
    pub(super) social_service: Arc<dyn SocialService>,
}

impl<Extensions> ApiRequestContent<Extensions> for LoginTelegramRequestContent
where
    Extensions: Resolve<Arc<dyn SocialService>>,
{
    type Data = LoginTelegramQuestion;

    fn create(origin_content: ApiRequestOriginContent<Self::Data, Extensions>) -> Self {
        Self {
            login_telegram_question: origin_content.data_result,
            social_service: origin_content.extensions.resolve(),
        }
    }
}
