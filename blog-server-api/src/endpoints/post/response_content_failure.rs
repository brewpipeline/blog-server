use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentFailure};

pub enum PostResponseContentFailure {
    DatabaseError { reason: String },
    SlugEmpty,
    NotFound,
}

impl ApiResponseContentBase for PostResponseContentFailure {
    fn status_code(&self) -> &'static StatusCode {
        match self {
            PostResponseContentFailure::DatabaseError { reason: _ } => {
                &StatusCode::INTERNAL_SERVER_ERROR
            }
            PostResponseContentFailure::SlugEmpty => &StatusCode::BAD_REQUEST,
            PostResponseContentFailure::NotFound => &StatusCode::NOT_FOUND,
        }
    }
}

impl ApiResponseContentFailure for PostResponseContentFailure {
    fn identifier(&self) -> &'static str {
        match self {
            PostResponseContentFailure::DatabaseError { reason: _ } => "POST_DATABASE_ERROR",
            PostResponseContentFailure::SlugEmpty => "POST_SLUG_EMPTY",
            PostResponseContentFailure::NotFound => "POST_NOT_FOUND",
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
            PostResponseContentFailure::SlugEmpty => {
                "post slug is empty in request URL".to_string()
            }
            PostResponseContentFailure::NotFound => "post record not found in database".to_string(),
        })
    }
}
