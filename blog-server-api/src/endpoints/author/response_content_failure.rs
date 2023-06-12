use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentFailure};

pub enum AuthorResponseContentFailure {
    DatabaseError { reason: String },
    NameMissing,
    NotFound,
}

impl ApiResponseContentBase for AuthorResponseContentFailure {
    fn status_code(&self) -> &'static StatusCode {
        match self {
            AuthorResponseContentFailure::DatabaseError { reason: _ } => {
                &StatusCode::INTERNAL_SERVER_ERROR
            }
            AuthorResponseContentFailure::NameMissing => &StatusCode::BAD_REQUEST,
            AuthorResponseContentFailure::NotFound => &StatusCode::BAD_REQUEST,
        }
    }
}

impl ApiResponseContentFailure for AuthorResponseContentFailure {
    fn identifier(&self) -> &'static str {
        match self {
            AuthorResponseContentFailure::DatabaseError { reason: _ } => "AUTHOR_DATABASE_ERROR",
            AuthorResponseContentFailure::NameMissing => "AUTHOR_NAME_MISSING",
            AuthorResponseContentFailure::NotFound => "AUTHOR_NOT_FOUND",
        }
    }

    fn reason(&self) -> Option<String> {
        Some(match self {
            AuthorResponseContentFailure::DatabaseError { reason } => {
                format!("database error: {}", reason)
            }
            AuthorResponseContentFailure::NameMissing => {
                "author name is missed in request URL".to_string()
            }
            AuthorResponseContentFailure::NotFound => {
                "author record not found in database".to_string()
            }
        })
    }
}
