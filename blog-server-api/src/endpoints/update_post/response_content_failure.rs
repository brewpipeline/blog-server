use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentFailure};

pub enum UpdatePostContentFailure {
    DatabaseError { reason: String },
    ValidationError { reason: String },
    Unauthorized { reason: String },
    IncorrectIdFormat { reason: String },
    PostNotFound,
    EditingForbidden,
}

impl ApiResponseContentBase for UpdatePostContentFailure {
    fn status_code(&self) -> &'static hyper::StatusCode {
        match self {
            UpdatePostContentFailure::DatabaseError { reason: _ } => {
                &StatusCode::INTERNAL_SERVER_ERROR
            }
            UpdatePostContentFailure::PostNotFound => &StatusCode::BAD_REQUEST,
            UpdatePostContentFailure::ValidationError { reason: _ } => &StatusCode::BAD_REQUEST,
            UpdatePostContentFailure::Unauthorized { reason: _ } => &StatusCode::UNAUTHORIZED,
            UpdatePostContentFailure::EditingForbidden => &StatusCode::FORBIDDEN,
            UpdatePostContentFailure::IncorrectIdFormat { reason: _ } => &StatusCode::BAD_REQUEST,
        }
    }
}

impl ApiResponseContentFailure for UpdatePostContentFailure {
    fn identifier(&self) -> &'static str {
        match self {
            UpdatePostContentFailure::DatabaseError { reason: _ } => "UPDATE_POST_DATABASE_ERROR",
            UpdatePostContentFailure::ValidationError { reason: _ } => {
                "UPDATE_POST_VALIDATION_ERROR"
            }
            UpdatePostContentFailure::PostNotFound => "UPDATE_POST_NOT_FOUND",
            UpdatePostContentFailure::Unauthorized { reason: _ } => "UPDATE_POST_UNAUTHORIZED",
            UpdatePostContentFailure::IncorrectIdFormat { reason: _ } => {
                "UPDATE_POST_INCORRECT_ID_FORMAT"
            }
            UpdatePostContentFailure::EditingForbidden => "UPDATE_POST_EDITING_FORBIDDEN",
        }
    }

    fn reason(&self) -> Option<String> {
        Some(match self {
            UpdatePostContentFailure::DatabaseError { reason } => {
                if cfg!(debug_assertions) {
                    format!("database error: {}", reason)
                } else {
                    "internal database error".to_string()
                }
            }
            UpdatePostContentFailure::Unauthorized { reason } => {
                if cfg!(debug_assertions) {
                    format!("unauthorized error: {}", reason)
                } else {
                    "unauthorized error".to_string()
                }
            }
            UpdatePostContentFailure::ValidationError { reason } => {
                format!("validation error: {}", reason)
            }
            UpdatePostContentFailure::PostNotFound => {
                String::from("post with specified ID not found")
            }
            UpdatePostContentFailure::IncorrectIdFormat { reason } => {
                format!("incorrect value provided for post ID: {}", reason)
            }
            UpdatePostContentFailure::EditingForbidden => {
                String::from("insufficient rights to edit post")
            }
        })
    }
}
