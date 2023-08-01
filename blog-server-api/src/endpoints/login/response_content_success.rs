use blog_generic::entities::LoginAnswer;
use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentSuccess};

#[derive(Debug, Clone)]
pub struct LoginResponseContentSuccess {
    login_answer: LoginAnswer,
}

impl Into<LoginResponseContentSuccess> for String {
    fn into(self) -> LoginResponseContentSuccess {
        LoginResponseContentSuccess {
            login_answer: LoginAnswer { token: self },
        }
    }
}

impl ApiResponseContentBase for LoginResponseContentSuccess {
    fn status_code(&self) -> &'static StatusCode {
        &StatusCode::OK
    }
}

impl ApiResponseContentSuccess for LoginResponseContentSuccess {
    type Data = LoginAnswer;

    fn identifier(&self) -> &'static str {
        "LOGIN_SUCCESS"
    }

    fn description(&self) -> Option<String> {
        Some("login success and token generated".to_string())
    }

    fn data(&self) -> &Self::Data {
        &self.login_answer
    }
}
