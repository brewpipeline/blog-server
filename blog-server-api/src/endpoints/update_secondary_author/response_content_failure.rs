use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentFailure};

pub enum UpdateSecondaryAuthorContentFailure {
    DatabaseError { reason: String },
    ValidationError { reason: String },
    Unauthorized { reason: String },
    EditingForbidden,
}

impl ApiResponseContentBase for UpdateSecondaryAuthorContentFailure {
    fn status_code(&self) -> &'static hyper::StatusCode {
        match self {
            UpdateSecondaryAuthorContentFailure::DatabaseError { reason: _ } => {
                &StatusCode::INTERNAL_SERVER_ERROR
            }
            UpdateSecondaryAuthorContentFailure::ValidationError { reason: _ } => {
                &StatusCode::BAD_REQUEST
            }
            UpdateSecondaryAuthorContentFailure::Unauthorized { reason: _ } => {
                &StatusCode::UNAUTHORIZED
            }
            UpdateSecondaryAuthorContentFailure::EditingForbidden => &StatusCode::FORBIDDEN,
        }
    }
}

impl ApiResponseContentFailure for UpdateSecondaryAuthorContentFailure {
    fn identifier(&self) -> &'static str {
        match self {
            UpdateSecondaryAuthorContentFailure::DatabaseError { reason: _ } => {
                "UPDATE_SECONDARY_AUTHOR_DATABASE_ERROR"
            }
            UpdateSecondaryAuthorContentFailure::ValidationError { reason: _ } => {
                "UPDATE_SECONDARY_AUTHOR_VALIDATION_ERROR"
            }
            UpdateSecondaryAuthorContentFailure::Unauthorized { reason: _ } => {
                "UPDATE_SECONDARY_AUTHOR_UNAUTHORIZED"
            }
            UpdateSecondaryAuthorContentFailure::EditingForbidden => {
                "UPDATE_SECONDARY_AUTHOR_EDITING_FORBIDDEN"
            }
        }
    }

    fn reason(&self) -> Option<String> {
        Some(match self {
            UpdateSecondaryAuthorContentFailure::DatabaseError { reason } => {
                if cfg!(debug_assertions) {
                    format!("database error: {}", reason)
                } else {
                    "internal database error".to_string()
                }
            }
            UpdateSecondaryAuthorContentFailure::Unauthorized { reason } => {
                if cfg!(debug_assertions) {
                    format!("unauthorized error: {}", reason)
                } else {
                    "unauthorized error".to_string()
                }
            }
            UpdateSecondaryAuthorContentFailure::ValidationError { reason } => {
                format!("validation error: {}", reason)
            }
            UpdateSecondaryAuthorContentFailure::EditingForbidden => {
                String::from("insufficient rights to edit author")
            }
        })
    }
}
