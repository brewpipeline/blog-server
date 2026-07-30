use blog_generic::entities::LoginAnswer;
use blog_server_api_macros::ApiSuccess;

#[derive(Debug, Clone, ApiSuccess)]
#[success(ok, description = "login yandex success and token generated")]
pub struct LoginYandexResponseContentSuccess {
    login_answer: LoginAnswer,
}

impl From<String> for LoginYandexResponseContentSuccess {
    fn from(value: String) -> Self {
        LoginYandexResponseContentSuccess {
            login_answer: LoginAnswer { token: value },
        }
    }
}
