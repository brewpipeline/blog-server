use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentFailure};

pub enum TagResponseContentFailure {
    DatabaseError { reason: String },
    NotFound,
    IncorrectIdFormat { reason: String },
}

impl ApiResponseContentBase for TagResponseContentFailure {
    fn status_code(&self) -> &'static StatusCode {
        match self {
            TagResponseContentFailure::DatabaseError { reason: _ } => {
                &StatusCode::INTERNAL_SERVER_ERROR
            }
            TagResponseContentFailure::NotFound => &StatusCode::NOT_FOUND,
            TagResponseContentFailure::IncorrectIdFormat { reason: _ } => &StatusCode::BAD_REQUEST,
        }
    }
}

impl ApiResponseContentFailure for TagResponseContentFailure {
    fn identifier(&self) -> &'static str {
        match self {
            TagResponseContentFailure::DatabaseError { reason: _ } => "TAG_DATABASE_ERROR",
            TagResponseContentFailure::NotFound => "TAG_NOT_FOUND",
            TagResponseContentFailure::IncorrectIdFormat { reason: _ } => "TAG_INCORRECT_ID_FORMAT",
        }
    }

    fn reason(&self) -> Option<String> {
        Some(match self {
            TagResponseContentFailure::DatabaseError { reason } => {
                if cfg!(debug_assertions) {
                    format!("database error: {}", reason)
                } else {
                    "internal database error".to_string()
                }
            }
            TagResponseContentFailure::NotFound => "tag record not found in database".to_string(),
            TagResponseContentFailure::IncorrectIdFormat { reason } => {
                format!("incorrect value provided for tag ID: {}", reason)
            }
        })
    }
}
