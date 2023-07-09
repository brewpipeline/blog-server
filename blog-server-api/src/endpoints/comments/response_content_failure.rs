use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentFailure};

pub enum CommentsResponseContentFailure {
    DatabaseError { reason: String },
    PostSlugEmpty,
    PostNotFound,
}

impl ApiResponseContentBase for CommentsResponseContentFailure {
    fn status_code(&self) -> &'static StatusCode {
        match self {
            CommentsResponseContentFailure::DatabaseError { reason: _ } => {
                &StatusCode::INTERNAL_SERVER_ERROR
            }
            CommentsResponseContentFailure::PostSlugEmpty => &StatusCode::BAD_REQUEST,
            CommentsResponseContentFailure::PostNotFound => &StatusCode::NOT_FOUND,
        }
    }
}

impl ApiResponseContentFailure for CommentsResponseContentFailure {
    fn identifier(&self) -> &'static str {
        match self {
            CommentsResponseContentFailure::DatabaseError { reason: _ } => {
                "COMMENTS_DATABASE_ERROR"
            }
            CommentsResponseContentFailure::PostSlugEmpty => "COMMENTS_POST_SLUG_EMPTY",
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
            CommentsResponseContentFailure::PostSlugEmpty => {
                "comments root post slug is empty in request URL".to_string()
            }
            CommentsResponseContentFailure::PostNotFound => {
                "comments root post record not found in database".to_string()
            }
        })
    }
}
