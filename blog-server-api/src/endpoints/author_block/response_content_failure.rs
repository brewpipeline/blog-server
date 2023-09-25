use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentFailure};

pub enum AuthorBlockResponseContentFailure {
    Unauthorized { reason: String },
    Forbidden,
    DatabaseError { reason: String },
    IncorrectIdFormat { reason: String },
}

impl ApiResponseContentBase for AuthorBlockResponseContentFailure {
    fn status_code(&self) -> &'static StatusCode {
        match self {
            AuthorBlockResponseContentFailure::Unauthorized { reason: _ } => {
                &StatusCode::UNAUTHORIZED
            }
            AuthorBlockResponseContentFailure::Forbidden => &StatusCode::FORBIDDEN,
            AuthorBlockResponseContentFailure::DatabaseError { reason: _ } => {
                &StatusCode::INTERNAL_SERVER_ERROR
            }
            AuthorBlockResponseContentFailure::IncorrectIdFormat { reason: _ } => {
                &StatusCode::BAD_REQUEST
            }
        }
    }
}

impl ApiResponseContentFailure for AuthorBlockResponseContentFailure {
    fn identifier(&self) -> &'static str {
        match self {
            AuthorBlockResponseContentFailure::Unauthorized { reason: _ } => {
                "AUTHOR_BLOCK_UNAUTHORIZED"
            }
            AuthorBlockResponseContentFailure::Forbidden => "AUTHOR_BLOCK_FORBIDDEN",
            AuthorBlockResponseContentFailure::DatabaseError { reason: _ } => {
                "AUTHOR_BLOCK_DATABASE_ERROR"
            }
            AuthorBlockResponseContentFailure::IncorrectIdFormat { reason: _ } => {
                "AUTHOR_BLOCK_INCORRECT_ID_FORMAT"
            }
        }
    }

    fn reason(&self) -> Option<String> {
        Some(match self {
            AuthorBlockResponseContentFailure::Unauthorized { reason } => {
                if cfg!(debug_assertions) {
                    format!("unauthorized error: {}", reason)
                } else {
                    "unauthorized error".to_string()
                }
            }
            AuthorBlockResponseContentFailure::Forbidden => String::from("insufficient rights"),
            AuthorBlockResponseContentFailure::DatabaseError { reason } => {
                if cfg!(debug_assertions) {
                    format!("database error: {}", reason)
                } else {
                    "internal database error".to_string()
                }
            }
            AuthorBlockResponseContentFailure::IncorrectIdFormat { reason } => {
                format!("incorrect value provided for author ID: {}", reason)
            }
        })
    }
}
