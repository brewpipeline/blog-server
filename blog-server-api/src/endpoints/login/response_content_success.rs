use blog_generic::entities::LoginAnswer;
use blog_server_api_macros::ApiSuccess;

#[derive(Debug, Clone, ApiSuccess)]
#[success(ok, from = String, description = "login success and token generated")]
pub struct LoginResponseContentSuccess {
    login_answer: LoginAnswer,
}
