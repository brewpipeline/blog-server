use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentSuccess};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct LoginResponseContentSuccess {
    token: String,
}

impl Into<LoginResponseContentSuccess> for String {
    fn into(self) -> LoginResponseContentSuccess {
        LoginResponseContentSuccess { token: self }
    }
}

impl ApiResponseContentBase for LoginResponseContentSuccess {
    fn status_code(&self) -> &'static StatusCode {
        &StatusCode::OK
    }
}

impl ApiResponseContentSuccess for LoginResponseContentSuccess {
    type Data = Self;

    fn identifier(&self) -> &'static str {
        "LOGIN_SUCCESS"
    }

    fn description(&self) -> Option<String> {
        Some("login success and token generated".to_string())
    }

    fn data(&self) -> &Self::Data {
        self
    }
}
