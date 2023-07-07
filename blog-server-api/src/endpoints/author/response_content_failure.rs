use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentFailure};

pub enum AuthorResponseContentFailure {
    DatabaseError { reason: String },
    SlugEmpty,
    NotFound,
}

impl ApiResponseContentBase for AuthorResponseContentFailure {
    fn status_code(&self) -> &'static StatusCode {
        match self {
            AuthorResponseContentFailure::DatabaseError { reason: _ } => {
                &StatusCode::INTERNAL_SERVER_ERROR
            }
            AuthorResponseContentFailure::SlugEmpty => &StatusCode::BAD_REQUEST,
            AuthorResponseContentFailure::NotFound => &StatusCode::NOT_FOUND,
        }
    }
}

impl ApiResponseContentFailure for AuthorResponseContentFailure {
    fn identifier(&self) -> &'static str {
        match self {
            AuthorResponseContentFailure::DatabaseError { reason: _ } => "AUTHOR_DATABASE_ERROR",
            AuthorResponseContentFailure::SlugEmpty => "AUTHOR_SLUG_EMPTY",
            AuthorResponseContentFailure::NotFound => "AUTHOR_NOT_FOUND",
        }
    }

    fn reason(&self) -> Option<String> {
        Some(match self {
            AuthorResponseContentFailure::DatabaseError { reason } => {
                if cfg!(debug_assertions) {
                    format!("database error: {}", reason)
                } else {
                    "internal database error".to_string()
                }
            }
            AuthorResponseContentFailure::SlugEmpty => {
                "author slug is empty in request URL".to_string()
            }
            AuthorResponseContentFailure::NotFound => {
                "author record not found in database".to_string()
            }
        })
    }
}
