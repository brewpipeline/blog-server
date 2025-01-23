use crate::extensions::Resolve;
use blog_generic::entities::LoginYandexQuestion;
use blog_server_services::traits::social_service::SocialService;
use screw_api::request::{ApiRequestContent, ApiRequestOriginContent};
use screw_components::dyn_result::DResult;
use std::sync::Arc;

pub struct LoginYandexRequestContent {
    pub(super) login_yandex_question: DResult<LoginYandexQuestion>,
    pub(super) social_service: Arc<dyn SocialService>,
}

impl<Extensions> ApiRequestContent<Extensions> for LoginYandexRequestContent
where
    Extensions: Resolve<Arc<dyn SocialService>>,
{
    type Data = LoginYandexQuestion;

    fn create(origin_content: ApiRequestOriginContent<Self::Data, Extensions>) -> Self {
        Self {
            login_yandex_question: origin_content.data_result,
            social_service: origin_content.extensions.resolve(),
        }
    }
}
