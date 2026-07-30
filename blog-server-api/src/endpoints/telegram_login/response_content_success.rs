use blog_generic::entities::LoginAnswer;
use blog_server_api_macros::ApiSuccess;

#[derive(Debug, Clone, ApiSuccess)]
#[success(ok, description = "login telegram success and token generated")]
pub struct LoginTelegramResponseContentSuccess {
    login_answer: LoginAnswer,
}

impl From<String> for LoginTelegramResponseContentSuccess {
    fn from(value: String) -> Self {
        LoginTelegramResponseContentSuccess {
            login_answer: LoginAnswer { token: value },
        }
    }
}
