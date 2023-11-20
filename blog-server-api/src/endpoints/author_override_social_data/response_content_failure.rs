use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentFailure};

pub enum AuthorOverrideSocialDataResponseContentFailure {
    Unauthorized { reason: String },
    Forbidden,
    DatabaseError { reason: String },
    IncorrectIdFormat { reason: String },
}

impl ApiResponseContentBase for AuthorOverrideSocialDataResponseContentFailure {
    fn status_code(&self) -> &'static StatusCode {
        match self {
            AuthorOverrideSocialDataResponseContentFailure::Unauthorized { reason: _ } => {
                &StatusCode::UNAUTHORIZED
            }
            AuthorOverrideSocialDataResponseContentFailure::Forbidden => &StatusCode::FORBIDDEN,
            AuthorOverrideSocialDataResponseContentFailure::DatabaseError { reason: _ } => {
                &StatusCode::INTERNAL_SERVER_ERROR
            }
            AuthorOverrideSocialDataResponseContentFailure::IncorrectIdFormat { reason: _ } => {
                &StatusCode::BAD_REQUEST
            }
        }
    }
}

impl ApiResponseContentFailure for AuthorOverrideSocialDataResponseContentFailure {
    fn identifier(&self) -> &'static str {
        match self {
            AuthorOverrideSocialDataResponseContentFailure::Unauthorized { reason: _ } => {
                "AUTHOR_OVERRIDE_SOCIAL_DATA_UNAUTHORIZED"
            }
            AuthorOverrideSocialDataResponseContentFailure::Forbidden => {
                "AUTHOR_OVERRIDE_SOCIAL_DATA_FORBIDDEN"
            }
            AuthorOverrideSocialDataResponseContentFailure::DatabaseError { reason: _ } => {
                "AUTHOR_OVERRIDE_SOCIAL_DATA_DATABASE_ERROR"
            }
            AuthorOverrideSocialDataResponseContentFailure::IncorrectIdFormat { reason: _ } => {
                "AUTHOR_OVERRIDE_SOCIAL_DATA_INCORRECT_ID_FORMAT"
            }
        }
    }

    fn reason(&self) -> Option<String> {
        Some(match self {
            AuthorOverrideSocialDataResponseContentFailure::Unauthorized { reason } => {
                if cfg!(debug_assertions) {
                    format!("unauthorized error: {}", reason)
                } else {
                    "unauthorized error".to_string()
                }
            }
            AuthorOverrideSocialDataResponseContentFailure::Forbidden => {
                String::from("insufficient rights")
            }
            AuthorOverrideSocialDataResponseContentFailure::DatabaseError { reason } => {
                if cfg!(debug_assertions) {
                    format!("database error: {}", reason)
                } else {
                    "internal database error".to_string()
                }
            }
            AuthorOverrideSocialDataResponseContentFailure::IncorrectIdFormat { reason } => {
                format!("incorrect value provided for author ID: {}", reason)
            }
        })
    }
}
