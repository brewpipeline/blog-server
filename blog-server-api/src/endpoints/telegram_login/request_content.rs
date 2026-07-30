use blog_generic::entities::LoginTelegramQuestion;
use blog_server_api_macros::ApiRequest;
use blog_server_services::traits::social_service::SocialService;
use screw_components::dyn_result::DResult;
use std::sync::Arc;

#[derive(ApiRequest)]
pub struct LoginTelegramRequestContent {
    #[request(data)]
    pub(super) login_telegram_question: DResult<LoginTelegramQuestion>,
    #[request(extension)]
    pub(super) social_service: Arc<dyn SocialService>,
}
