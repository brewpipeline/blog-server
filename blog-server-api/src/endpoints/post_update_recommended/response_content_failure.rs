use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentFailure};

pub enum PostUpdateRecommendedResponseContentFailure {
    Unauthorized { reason: String },
    Forbidden,
    DatabaseError { reason: String },
    IncorrectIdFormat { reason: String },
}

impl ApiResponseContentBase for PostUpdateRecommendedResponseContentFailure {
    fn status_code(&self) -> &'static StatusCode {
        match self {
            PostUpdateRecommendedResponseContentFailure::Unauthorized { reason: _ } => {
                &StatusCode::UNAUTHORIZED
            }
            PostUpdateRecommendedResponseContentFailure::Forbidden => &StatusCode::FORBIDDEN,
            PostUpdateRecommendedResponseContentFailure::DatabaseError { reason: _ } => {
                &StatusCode::INTERNAL_SERVER_ERROR
            }
            PostUpdateRecommendedResponseContentFailure::IncorrectIdFormat { reason: _ } => {
                &StatusCode::BAD_REQUEST
            }
        }
    }
}

impl ApiResponseContentFailure for PostUpdateRecommendedResponseContentFailure {
    fn identifier(&self) -> &'static str {
        match self {
            PostUpdateRecommendedResponseContentFailure::Unauthorized { reason: _ } => {
                "POST_UPDATE_RECOMMENDED_UNAUTHORIZED"
            }
            PostUpdateRecommendedResponseContentFailure::Forbidden => {
                "POST_UPDATE_RECOMMENDED_FORBIDDEN"
            }
            PostUpdateRecommendedResponseContentFailure::DatabaseError { reason: _ } => {
                "POST_UPDATE_RECOMMENDED_DATABASE_ERROR"
            }
            PostUpdateRecommendedResponseContentFailure::IncorrectIdFormat { reason: _ } => {
                "POST_UPDATE_RECOMMENDED_INCORRECT_ID_FORMAT"
            }
        }
    }

    fn reason(&self) -> Option<String> {
        Some(match self {
            PostUpdateRecommendedResponseContentFailure::Unauthorized { reason } => {
                if cfg!(debug_assertions) {
                    format!("unauthorized error: {}", reason)
                } else {
                    "unauthorized error".to_string()
                }
            }
            PostUpdateRecommendedResponseContentFailure::Forbidden => {
                String::from("insufficient rights")
            }
            PostUpdateRecommendedResponseContentFailure::DatabaseError { reason } => {
                if cfg!(debug_assertions) {
                    format!("database error: {}", reason)
                } else {
                    "internal database error".to_string()
                }
            }
            PostUpdateRecommendedResponseContentFailure::IncorrectIdFormat { reason } => {
                format!("incorrect value provided for post ID: {}", reason)
            }
        })
    }
}
