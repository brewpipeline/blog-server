use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentFailure};

pub enum PostResponseContentFailure {
    DatabaseError { reason: String },
    ZeroId,
    NotFound,
    IncorrectIdFormat { reason: String },
}

impl ApiResponseContentBase for PostResponseContentFailure {
    fn status_code(&self) -> &'static StatusCode {
        match self {
            PostResponseContentFailure::DatabaseError { reason: _ } => {
                &StatusCode::INTERNAL_SERVER_ERROR
            }
            PostResponseContentFailure::ZeroId => &StatusCode::BAD_REQUEST,
            PostResponseContentFailure::NotFound => &StatusCode::NOT_FOUND,
            PostResponseContentFailure::IncorrectIdFormat { reason: _ } => &StatusCode::BAD_REQUEST,
        }
    }
}

impl ApiResponseContentFailure for PostResponseContentFailure {
    fn identifier(&self) -> &'static str {
        match self {
            PostResponseContentFailure::DatabaseError { reason: _ } => "POST_DATABASE_ERROR",
            PostResponseContentFailure::ZeroId => "POST_ZERO_ID",
            PostResponseContentFailure::NotFound => "POST_NOT_FOUND",
            PostResponseContentFailure::IncorrectIdFormat { reason: _ } => {
                "POST_INCORRECT_ID_FORMAT"
            }
        }
    }

    fn reason(&self) -> Option<String> {
        Some(match self {
            PostResponseContentFailure::DatabaseError { reason } => {
                if cfg!(debug_assertions) {
                    format!("database error: {}", reason)
                } else {
                    "internal database error".to_string()
                }
            }
            PostResponseContentFailure::ZeroId => "id is 0 in request URL".to_string(),
            PostResponseContentFailure::NotFound => "post record not found in database".to_string(),
            PostResponseContentFailure::IncorrectIdFormat { reason } => {
                format!("incorrect value provided for post ID: {}", reason)
            }
        })
    }
}
