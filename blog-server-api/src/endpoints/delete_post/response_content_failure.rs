use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentFailure};

pub enum DeletePostResponseContentFailure {
    DatabaseError { reason: String },
    NotFound,
    IncorrectIdFormat { reason: String },
    Unauthorized { reason: String },
    EditingForbidden,
}

impl ApiResponseContentBase for DeletePostResponseContentFailure {
    fn status_code(&self) -> &'static StatusCode {
        match self {
            DeletePostResponseContentFailure::DatabaseError { reason: _ } => {
                &StatusCode::INTERNAL_SERVER_ERROR
            }
            DeletePostResponseContentFailure::NotFound => &StatusCode::NOT_FOUND,
            DeletePostResponseContentFailure::IncorrectIdFormat { reason: _ } => {
                &StatusCode::BAD_REQUEST
            }
            DeletePostResponseContentFailure::Unauthorized { reason: _ } => {
                &StatusCode::UNAUTHORIZED
            }
            DeletePostResponseContentFailure::EditingForbidden => &StatusCode::FORBIDDEN,
        }
    }
}

impl ApiResponseContentFailure for DeletePostResponseContentFailure {
    fn identifier(&self) -> &'static str {
        match self {
            DeletePostResponseContentFailure::DatabaseError { reason: _ } => {
                "DELETE_POST_DATABASE_ERROR"
            }
            DeletePostResponseContentFailure::NotFound => "DELETE_POST_NOT_FOUND",
            DeletePostResponseContentFailure::IncorrectIdFormat { reason: _ } => {
                "DELETE_POST_INCORRECT_ID_FORMAT"
            }
            DeletePostResponseContentFailure::Unauthorized { reason: _ } => {
                "DELETE_POST_UNAUTHORIZED"
            }
            DeletePostResponseContentFailure::EditingForbidden => "DELETE_POST_DELETING_FORBIDDEN",
        }
    }

    fn reason(&self) -> Option<String> {
        Some(match self {
            DeletePostResponseContentFailure::DatabaseError { reason } => {
                if cfg!(debug_assertions) {
                    format!("database error: {}", reason)
                } else {
                    "internal database error".to_string()
                }
            }
            DeletePostResponseContentFailure::NotFound => {
                "post record not found in database".to_string()
            }
            DeletePostResponseContentFailure::IncorrectIdFormat { reason } => {
                format!("incorrect value provided for post ID: {}", reason)
            }
            DeletePostResponseContentFailure::Unauthorized { reason } => {
                if cfg!(debug_assertions) {
                    format!("unauthorized error: {}", reason)
                } else {
                    "unauthorized error".to_string()
                }
            }
            DeletePostResponseContentFailure::EditingForbidden => {
                String::from("insufficient rights to delete post")
            }
        })
    }
}
