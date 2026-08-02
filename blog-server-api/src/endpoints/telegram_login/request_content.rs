use blog_generic::entities::LoginTelegramQuestion;
use blog_server_api_macros::ApiRequest;
use blog_server_services::traits::social_service::SocialService;
use std::sync::Arc;

#[derive(ApiRequest)]
#[request(failure = super::response_content_failure::LoginTelegramResponseContentFailure)]
pub struct LoginTelegramRequestContent {
    #[request(data)]
    pub(super) login_telegram_question: LoginTelegramQuestion,
    #[request(extension)]
    pub(super) social_service: Arc<dyn SocialService>,
}
