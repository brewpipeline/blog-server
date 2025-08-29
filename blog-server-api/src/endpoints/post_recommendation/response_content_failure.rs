use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentFailure};

pub enum PostRecommendationResponseContentFailure {
    DatabaseError { reason: String },
    NotFound,
    IncorrectIdFormat { reason: String },
}

impl ApiResponseContentBase for PostRecommendationResponseContentFailure {
    fn status_code(&self) -> &'static StatusCode {
        match self {
            PostRecommendationResponseContentFailure::DatabaseError { reason: _ } => {
                &StatusCode::INTERNAL_SERVER_ERROR
            }
            PostRecommendationResponseContentFailure::NotFound => &StatusCode::NOT_FOUND,
            PostRecommendationResponseContentFailure::IncorrectIdFormat { reason: _ } => {
                &StatusCode::BAD_REQUEST
            }
        }
    }
}

impl ApiResponseContentFailure for PostRecommendationResponseContentFailure {
    fn identifier(&self) -> &'static str {
        match self {
            PostRecommendationResponseContentFailure::DatabaseError { reason: _ } => {
                "POST_RECOMMENDATION_DATABASE_ERROR"
            }
            PostRecommendationResponseContentFailure::NotFound => "POST_RECOMMENDATION_NOT_FOUND",
            PostRecommendationResponseContentFailure::IncorrectIdFormat { reason: _ } => {
                "POST_RECOMMENDATION_INCORRECT_ID_FORMAT"
            }
        }
    }

    fn reason(&self) -> Option<String> {
        Some(match self {
            PostRecommendationResponseContentFailure::DatabaseError { reason } => {
                if cfg!(debug_assertions) {
                    format!("database error: {}", reason)
                } else {
                    "internal database error".to_string()
                }
            }
            PostRecommendationResponseContentFailure::NotFound => {
                "post recommendation not found".to_string()
            }
            PostRecommendationResponseContentFailure::IncorrectIdFormat { reason } => {
                format!("incorrect value provided for post ID: {}", reason)
            }
        })
    }
}
