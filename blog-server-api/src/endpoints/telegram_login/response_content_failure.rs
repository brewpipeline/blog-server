use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentFailure};

pub enum LoginTelegramResponseContentFailure {
    DatabaseError { reason: String },
    ParamsDecodeError { reason: String },
    TokenGeneratingError { reason: String },
    TelegramError { reason: String },
}

impl ApiResponseContentBase for LoginTelegramResponseContentFailure {
    fn status_code(&self) -> &'static StatusCode {
        match self {
            LoginTelegramResponseContentFailure::DatabaseError { reason: _ } => {
                &StatusCode::INTERNAL_SERVER_ERROR
            }
            LoginTelegramResponseContentFailure::ParamsDecodeError { reason: _ } => {
                &StatusCode::BAD_REQUEST
            }
            LoginTelegramResponseContentFailure::TokenGeneratingError { reason: _ } => {
                &StatusCode::INTERNAL_SERVER_ERROR
            }
            LoginTelegramResponseContentFailure::TelegramError { reason: _ } => {
                &StatusCode::BAD_REQUEST
            }
        }
    }
}

impl ApiResponseContentFailure for LoginTelegramResponseContentFailure {
    fn identifier(&self) -> &'static str {
        match self {
            LoginTelegramResponseContentFailure::DatabaseError { reason: _ } => {
                "LOGIN_TELEGRAM_DATABASE_ERROR"
            }
            LoginTelegramResponseContentFailure::ParamsDecodeError { reason: _ } => {
                "LOGIN_TELEGRAM_PARAMS_ERROR"
            }
            LoginTelegramResponseContentFailure::TokenGeneratingError { reason: _ } => {
                "LOGIN_TELEGRAM_TOKEN_GENERATING_ERROR"
            }
            LoginTelegramResponseContentFailure::TelegramError { reason: _ } => {
                "LOGIN_TELEGRAM_VENDOR_ERROR"
            }
        }
    }

    fn reason(&self) -> Option<String> {
        Some(match self {
            LoginTelegramResponseContentFailure::DatabaseError { reason } => {
                if cfg!(debug_assertions) {
                    format!("database error: {}", reason)
                } else {
                    "internal database error".to_string()
                }
            }
            LoginTelegramResponseContentFailure::ParamsDecodeError { reason } => {
                format!("params error: {}", reason)
            }
            LoginTelegramResponseContentFailure::TokenGeneratingError { reason } => {
                if cfg!(debug_assertions) {
                    format!("token generating error: {}", reason)
                } else {
                    "internal token generating error".to_string()
                }
            }
            LoginTelegramResponseContentFailure::TelegramError { reason } => {
                if cfg!(debug_assertions) {
                    format!("telegram vendor error: {}", reason)
                } else {
                    "internal telegram vendor error".to_string()
                }
            }
        })
    }
}
