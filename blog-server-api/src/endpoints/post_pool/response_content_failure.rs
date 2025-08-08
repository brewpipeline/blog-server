use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentFailure};

pub enum PostPoolResponseContentFailure {
    Unauthorized { reason: String },
    Forbidden,
    DatabaseError { reason: String },
    IncorrectIdFormat { reason: String },
}

impl ApiResponseContentBase for PostPoolResponseContentFailure {
    fn status_code(&self) -> &'static StatusCode {
        match self {
            PostPoolResponseContentFailure::Unauthorized { reason: _ } => &StatusCode::UNAUTHORIZED,
            PostPoolResponseContentFailure::Forbidden => &StatusCode::FORBIDDEN,
            PostPoolResponseContentFailure::DatabaseError { reason: _ } => {
                &StatusCode::INTERNAL_SERVER_ERROR
            }
            PostPoolResponseContentFailure::IncorrectIdFormat { reason: _ } => {
                &StatusCode::BAD_REQUEST
            }
        }
    }
}

impl ApiResponseContentFailure for PostPoolResponseContentFailure {
    fn identifier(&self) -> &'static str {
        match self {
            PostPoolResponseContentFailure::Unauthorized { reason: _ } => "POST_POOL_UNAUTHORIZED",
            PostPoolResponseContentFailure::Forbidden => "POST_POOL_FORBIDDEN",
            PostPoolResponseContentFailure::DatabaseError { reason: _ } => {
                "POST_POOL_DATABASE_ERROR"
            }
            PostPoolResponseContentFailure::IncorrectIdFormat { reason: _ } => {
                "POST_POOL_INCORRECT_ID_FORMAT"
            }
        }
    }

    fn reason(&self) -> Option<String> {
        Some(match self {
            PostPoolResponseContentFailure::Unauthorized { reason } => {
                if cfg!(debug_assertions) {
                    format!("unauthorized error: {}", reason)
                } else {
                    "unauthorized error".to_string()
                }
            }
            PostPoolResponseContentFailure::Forbidden => String::from("insufficient rights"),
            PostPoolResponseContentFailure::DatabaseError { reason } => {
                if cfg!(debug_assertions) {
                    format!("database error: {}", reason)
                } else {
                    "internal database error".to_string()
                }
            }
            PostPoolResponseContentFailure::IncorrectIdFormat { reason } => {
                format!("incorrect value provided for post ID: {}", reason)
            }
        })
    }
}
