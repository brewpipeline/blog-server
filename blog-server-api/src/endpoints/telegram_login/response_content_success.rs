use blog_generic::entities::LoginAnswer;
use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentSuccess};

#[derive(Debug, Clone)]
pub struct LoginTelegramResponseContentSuccess {
    login_answer: LoginAnswer,
}

impl Into<LoginTelegramResponseContentSuccess> for String {
    fn into(self) -> LoginTelegramResponseContentSuccess {
        LoginTelegramResponseContentSuccess {
            login_answer: LoginAnswer { token: self },
        }
    }
}

impl ApiResponseContentBase for LoginTelegramResponseContentSuccess {
    fn status_code(&self) -> &'static StatusCode {
        &StatusCode::OK
    }
}

impl ApiResponseContentSuccess for LoginTelegramResponseContentSuccess {
    type Data = LoginAnswer;

    fn identifier(&self) -> &'static str {
        "LOGIN_TELEGRAM_SUCCESS"
    }

    fn description(&self) -> Option<String> {
        Some("login telegram success and token generated".to_string())
    }

    fn data(&self) -> &Self::Data {
        &self.login_answer
    }
}
