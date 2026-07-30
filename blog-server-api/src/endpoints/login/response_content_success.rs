use blog_generic::entities::LoginAnswer;
use blog_server_api_macros::ApiSuccess;

#[derive(Debug, Clone, ApiSuccess)]
#[success(ok, description = "login success and token generated")]
pub struct LoginResponseContentSuccess {
    login_answer: LoginAnswer,
}

impl From<String> for LoginResponseContentSuccess {
    fn from(value: String) -> Self {
        LoginResponseContentSuccess {
            login_answer: LoginAnswer { token: value },
        }
    }
}
