use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentFailure};

pub enum LoginResponseContentFailure {
    DatabaseError { reason: String },
    ParamsDecodeError { reason: String },
    NameEmpty,
    NotFound,
    PasswordVerificationError { reason: String },
    WrongPassword,
    TokenGeneratingError { reason: String },
}

impl ApiResponseContentBase for LoginResponseContentFailure {
    fn status_code(&self) -> &'static StatusCode {
        match self {
            LoginResponseContentFailure::DatabaseError { reason: _ } => {
                &StatusCode::INTERNAL_SERVER_ERROR
            }
            LoginResponseContentFailure::ParamsDecodeError { reason: _ } => {
                &StatusCode::BAD_REQUEST
            }
            LoginResponseContentFailure::NameEmpty => &StatusCode::BAD_REQUEST,
            LoginResponseContentFailure::NotFound => &StatusCode::NOT_FOUND,
            LoginResponseContentFailure::PasswordVerificationError { reason: _ } => {
                &StatusCode::INTERNAL_SERVER_ERROR
            }
            LoginResponseContentFailure::WrongPassword => &StatusCode::FORBIDDEN,
            LoginResponseContentFailure::TokenGeneratingError { reason: _ } => {
                &StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

impl ApiResponseContentFailure for LoginResponseContentFailure {
    fn identifier(&self) -> &'static str {
        match self {
            LoginResponseContentFailure::DatabaseError { reason: _ } => "LOGIN_DATABASE_ERROR",
            LoginResponseContentFailure::ParamsDecodeError { reason: _ } => "LOGIN_PARAMS_ERROR",
            LoginResponseContentFailure::NameEmpty => "LOGIN_AUTHOR_NAME_EMPTY",
            LoginResponseContentFailure::NotFound => "LOGIN_AUTHOR_NOT_FOUND",
            LoginResponseContentFailure::PasswordVerificationError { reason: _ } => {
                "LOGIN_PASSWORD_VERIFICATION_ERROR"
            }
            LoginResponseContentFailure::WrongPassword => "LOGIN_WRONG_PASSWORD",
            LoginResponseContentFailure::TokenGeneratingError { reason: _ } => {
                "LOGIN_TOKEN_GENERATING_ERROR"
            }
        }
    }

    fn reason(&self) -> Option<String> {
        Some(match self {
            LoginResponseContentFailure::DatabaseError { reason } => {
                if cfg!(debug_assertions) {
                    format!("database error: {}", reason)
                } else {
                    "internal database error".to_string()
                }
            }
            LoginResponseContentFailure::ParamsDecodeError { reason } => {
                format!("params error: {}", reason)
            }
            LoginResponseContentFailure::NameEmpty => "author name is empty in params".to_string(),
            LoginResponseContentFailure::NotFound => {
                "author record not found in database".to_string()
            }
            LoginResponseContentFailure::PasswordVerificationError { reason } => {
                if cfg!(debug_assertions) {
                    format!("password verification error: {}", reason)
                } else {
                    "internal password verification error".to_string()
                }
            }
            LoginResponseContentFailure::WrongPassword => {
                "wrong password passed to request".to_string()
            }
            LoginResponseContentFailure::TokenGeneratingError { reason } => {
                if cfg!(debug_assertions) {
                    format!("token generating error: {}", reason)
                } else {
                    "internal token generating error".to_string()
                }
            }
        })
    }
}
