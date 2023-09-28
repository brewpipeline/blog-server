use blog_generic::entities::LoginAnswer;
use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentSuccess};

#[derive(Debug, Clone)]
pub struct LoginYandexResponseContentSuccess {
    login_answer: LoginAnswer,
}

impl Into<LoginYandexResponseContentSuccess> for String {
    fn into(self) -> LoginYandexResponseContentSuccess {
        LoginYandexResponseContentSuccess {
            login_answer: LoginAnswer { token: self },
        }
    }
}

impl ApiResponseContentBase for LoginYandexResponseContentSuccess {
    fn status_code(&self) -> &'static StatusCode {
        &StatusCode::OK
    }
}

impl ApiResponseContentSuccess for LoginYandexResponseContentSuccess {
    type Data = LoginAnswer;

    fn identifier(&self) -> &'static str {
        "LOGIN_YANDEX_SUCCESS"
    }

    fn description(&self) -> Option<String> {
        Some("login yandex success and token generated".to_string())
    }

    fn data(&self) -> &Self::Data {
        &self.login_answer
    }
}
