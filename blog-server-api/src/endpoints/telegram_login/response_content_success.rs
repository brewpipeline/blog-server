use blog_generic::entities::LoginAnswer;
use blog_server_api_macros::ApiSuccess;

#[derive(Debug, Clone, ApiSuccess)]
#[success(ok, from = String, description = "login telegram success and token generated")]
pub struct LoginTelegramResponseContentSuccess {
    login_answer: LoginAnswer,
}
