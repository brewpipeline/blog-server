use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentFailure};

pub enum AuthorsResponseContentFailure {
    DatabaseError { reason: String },
}

impl ApiResponseContentBase for AuthorsResponseContentFailure {
    fn status_code(&self) -> &'static StatusCode {
        match self {
            AuthorsResponseContentFailure::DatabaseError { reason: _ } => {
                &StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

impl ApiResponseContentFailure for AuthorsResponseContentFailure {
    fn identifier(&self) -> &'static str {
        match self {
            AuthorsResponseContentFailure::DatabaseError { reason: _ } => "AUTHORS_DATABASE_ERROR",
        }
    }

    fn reason(&self) -> Option<String> {
        Some(match self {
            AuthorsResponseContentFailure::DatabaseError { reason } => {
                if cfg!(debug_assertions) {
                    format!("database error: {}", reason)
                } else {
                    "internal database error".to_string()
                }
            }
        })
    }
}
