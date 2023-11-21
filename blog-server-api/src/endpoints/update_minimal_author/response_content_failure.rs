use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentFailure};

pub enum UpdateMinimalAuthorContentFailure {
    DatabaseError { reason: String },
    ValidationError { reason: String },
    Unauthorized { reason: String },
    EditingForbidden,
}

impl ApiResponseContentBase for UpdateMinimalAuthorContentFailure {
    fn status_code(&self) -> &'static hyper::StatusCode {
        match self {
            UpdateMinimalAuthorContentFailure::DatabaseError { reason: _ } => {
                &StatusCode::INTERNAL_SERVER_ERROR
            }
            UpdateMinimalAuthorContentFailure::ValidationError { reason: _ } => {
                &StatusCode::BAD_REQUEST
            }
            UpdateMinimalAuthorContentFailure::Unauthorized { reason: _ } => {
                &StatusCode::UNAUTHORIZED
            }
            UpdateMinimalAuthorContentFailure::EditingForbidden => &StatusCode::FORBIDDEN,
        }
    }
}

impl ApiResponseContentFailure for UpdateMinimalAuthorContentFailure {
    fn identifier(&self) -> &'static str {
        match self {
            UpdateMinimalAuthorContentFailure::DatabaseError { reason: _ } => {
                "UPDATE_MINIMAL_AUTHOR_DATABASE_ERROR"
            }
            UpdateMinimalAuthorContentFailure::ValidationError { reason: _ } => {
                "UPDATE_MINIMAL_AUTHOR_VALIDATION_ERROR"
            }
            UpdateMinimalAuthorContentFailure::Unauthorized { reason: _ } => {
                "UPDATE_MINIMAL_AUTHOR_UNAUTHORIZED"
            }
            UpdateMinimalAuthorContentFailure::EditingForbidden => {
                "UPDATE_MINIMAL_AUTHOR_EDITING_FORBIDDEN"
            }
        }
    }

    fn reason(&self) -> Option<String> {
        Some(match self {
            UpdateMinimalAuthorContentFailure::DatabaseError { reason } => {
                if cfg!(debug_assertions) {
                    format!("database error: {}", reason)
                } else {
                    "internal database error".to_string()
                }
            }
            UpdateMinimalAuthorContentFailure::Unauthorized { reason } => {
                if cfg!(debug_assertions) {
                    format!("unauthorized error: {}", reason)
                } else {
                    "unauthorized error".to_string()
                }
            }
            UpdateMinimalAuthorContentFailure::ValidationError { reason } => {
                format!("validation error: {}", reason)
            }
            UpdateMinimalAuthorContentFailure::EditingForbidden => {
                String::from("insufficient rights to edit author")
            }
        })
    }
}
