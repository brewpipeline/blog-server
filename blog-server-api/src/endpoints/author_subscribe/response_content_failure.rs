use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentFailure};

pub enum AuthorSubscribeResponseContentFailure {
    Unauthorized { reason: String },
    Forbidden,
    DatabaseError { reason: String },
    IncorrectIdFormat { reason: String },
    NotFound,
}

impl ApiResponseContentBase for AuthorSubscribeResponseContentFailure {
    fn status_code(&self) -> &'static StatusCode {
        match self {
            AuthorSubscribeResponseContentFailure::Unauthorized { reason: _ } => {
                &StatusCode::UNAUTHORIZED
            }
            AuthorSubscribeResponseContentFailure::Forbidden => &StatusCode::FORBIDDEN,
            AuthorSubscribeResponseContentFailure::DatabaseError { reason: _ } => {
                &StatusCode::INTERNAL_SERVER_ERROR
            }
            AuthorSubscribeResponseContentFailure::IncorrectIdFormat { reason: _ } => {
                &StatusCode::BAD_REQUEST
            }
            AuthorSubscribeResponseContentFailure::NotFound => &StatusCode::NOT_FOUND,
        }
    }
}

impl ApiResponseContentFailure for AuthorSubscribeResponseContentFailure {
    fn identifier(&self) -> &'static str {
        match self {
            AuthorSubscribeResponseContentFailure::Unauthorized { reason: _ } => {
                "AUTHOR_SUBSCRIBE_UNAUTHORIZED"
            }
            AuthorSubscribeResponseContentFailure::DatabaseError { reason: _ } => {
                "AUTHOR_SUBSCRIBE_DATABASE_ERROR"
            }
            AuthorSubscribeResponseContentFailure::Forbidden => "AUTHOR_SUBSCRIBE_FORBIDDEN",
            AuthorSubscribeResponseContentFailure::IncorrectIdFormat { reason: _ } => {
                "AUTHOR_SUBSCRIBE_INCORRECT_ID_FORMAT"
            }
            AuthorSubscribeResponseContentFailure::NotFound => "AUTHOR_SUBSCRIBE_NOT_FOUND",
        }
    }

    fn reason(&self) -> Option<String> {
        Some(match self {
            AuthorSubscribeResponseContentFailure::Unauthorized { reason } => {
                if cfg!(debug_assertions) {
                    format!("unauthorized error: {}", reason)
                } else {
                    "unauthorized error".to_string()
                }
            }
            AuthorSubscribeResponseContentFailure::DatabaseError { reason } => {
                if cfg!(debug_assertions) {
                    format!("database error: {}", reason)
                } else {
                    "internal database error".to_string()
                }
            }
            AuthorSubscribeResponseContentFailure::Forbidden => String::from("insufficient rights"),
            AuthorSubscribeResponseContentFailure::IncorrectIdFormat { reason } => {
                format!("incorrect value provided for author ID: {}", reason)
            }
            AuthorSubscribeResponseContentFailure::NotFound => {
                "author record not found in database".to_string()
            }
        })
    }
}
