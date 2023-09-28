use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentFailure};

pub enum LoginYandexResponseContentFailure {
    DatabaseError { reason: String },
    ParamsDecodeError { reason: String },
    TokenGeneratingError { reason: String },
    YandexError { reason: String },
}

impl ApiResponseContentBase for LoginYandexResponseContentFailure {
    fn status_code(&self) -> &'static StatusCode {
        match self {
            LoginYandexResponseContentFailure::DatabaseError { reason: _ } => {
                &StatusCode::INTERNAL_SERVER_ERROR
            }
            LoginYandexResponseContentFailure::ParamsDecodeError { reason: _ } => {
                &StatusCode::BAD_REQUEST
            }
            LoginYandexResponseContentFailure::TokenGeneratingError { reason: _ } => {
                &StatusCode::INTERNAL_SERVER_ERROR
            }
            LoginYandexResponseContentFailure::YandexError { reason: _ } => {
                &StatusCode::BAD_REQUEST
            }
        }
    }
}

impl ApiResponseContentFailure for LoginYandexResponseContentFailure {
    fn identifier(&self) -> &'static str {
        match self {
            LoginYandexResponseContentFailure::DatabaseError { reason: _ } => {
                "LOGIN_YANDEX_DATABASE_ERROR"
            }
            LoginYandexResponseContentFailure::ParamsDecodeError { reason: _ } => {
                "LOGIN_YANDEX_PARAMS_ERROR"
            }
            LoginYandexResponseContentFailure::TokenGeneratingError { reason: _ } => {
                "LOGIN_YANDEX_TOKEN_GENERATING_ERROR"
            }
            LoginYandexResponseContentFailure::YandexError { reason: _ } => {
                "LOGIN_YANDEX_VENDOR_ERROR"
            }
        }
    }

    fn reason(&self) -> Option<String> {
        Some(match self {
            LoginYandexResponseContentFailure::DatabaseError { reason } => {
                if cfg!(debug_assertions) {
                    format!("database error: {}", reason)
                } else {
                    "internal database error".to_string()
                }
            }
            LoginYandexResponseContentFailure::ParamsDecodeError { reason } => {
                format!("params error: {}", reason)
            }
            LoginYandexResponseContentFailure::TokenGeneratingError { reason } => {
                if cfg!(debug_assertions) {
                    format!("token generating error: {}", reason)
                } else {
                    "internal token generating error".to_string()
                }
            }
            LoginYandexResponseContentFailure::YandexError { reason } => {
                if cfg!(debug_assertions) {
                    format!("yandex vendor error: {}", reason)
                } else {
                    "internal yandex vendor error".to_string()
                }
            }
        })
    }
}
