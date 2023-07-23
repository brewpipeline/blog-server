use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentFailure};

pub enum CommentsResponseContentFailure {
    DatabaseError { reason: String },
    IncorrectIdFormat { reason: String },
    PostIdZero,
    PostNotFound,
}

impl ApiResponseContentBase for CommentsResponseContentFailure {
    fn status_code(&self) -> &'static StatusCode {
        match self {
            CommentsResponseContentFailure::DatabaseError { reason: _ } => {
                &StatusCode::INTERNAL_SERVER_ERROR
            }
            CommentsResponseContentFailure::PostIdZero => &StatusCode::BAD_REQUEST,
            CommentsResponseContentFailure::PostNotFound => &StatusCode::NOT_FOUND,
            CommentsResponseContentFailure::IncorrectIdFormat { reason: _ } => {
                &StatusCode::BAD_REQUEST
            }
        }
    }
}

impl ApiResponseContentFailure for CommentsResponseContentFailure {
    fn identifier(&self) -> &'static str {
        match self {
            CommentsResponseContentFailure::DatabaseError { reason: _ } => {
                "COMMENTS_DATABASE_ERROR"
            }
            CommentsResponseContentFailure::IncorrectIdFormat { reason: _ } => {
                "COMMENTS_INCORRECT_ID_FORMAT"
            }
            CommentsResponseContentFailure::PostIdZero => "COMMENTS_POST_ID_ZERO",
            CommentsResponseContentFailure::PostNotFound => "COMMENTS_POST_NOT_FOUND",
        }
    }

    fn reason(&self) -> Option<String> {
        Some(match self {
            CommentsResponseContentFailure::DatabaseError { reason } => {
                if cfg!(debug_assertions) {
                    format!("database error: {}", reason)
                } else {
                    "internal database error".to_string()
                }
            }
            CommentsResponseContentFailure::PostIdZero => {
                "comments root post id is zero in request URL".to_string()
            }
            CommentsResponseContentFailure::PostNotFound => {
                "comments root post record not found in database".to_string()
            }
            CommentsResponseContentFailure::IncorrectIdFormat { reason } => {
                format!("incorrect value provided for post ID: {}", reason)
            }
        })
    }
}
