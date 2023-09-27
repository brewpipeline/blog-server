use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentFailure};

pub enum DeleteCommentResponseContentFailure {
    DatabaseError { reason: String },
    NotFound,
    IncorrectIdFormat { reason: String },
    Unauthorized { reason: String },
    EditingForbidden,
}

impl ApiResponseContentBase for DeleteCommentResponseContentFailure {
    fn status_code(&self) -> &'static StatusCode {
        match self {
            DeleteCommentResponseContentFailure::DatabaseError { reason: _ } => {
                &StatusCode::INTERNAL_SERVER_ERROR
            }
            DeleteCommentResponseContentFailure::NotFound => &StatusCode::NOT_FOUND,
            DeleteCommentResponseContentFailure::IncorrectIdFormat { reason: _ } => {
                &StatusCode::BAD_REQUEST
            }
            DeleteCommentResponseContentFailure::Unauthorized { reason: _ } => {
                &StatusCode::UNAUTHORIZED
            }
            DeleteCommentResponseContentFailure::EditingForbidden => &StatusCode::FORBIDDEN,
        }
    }
}

impl ApiResponseContentFailure for DeleteCommentResponseContentFailure {
    fn identifier(&self) -> &'static str {
        match self {
            DeleteCommentResponseContentFailure::DatabaseError { reason: _ } => {
                "DELETE_COMMENT_DATABASE_ERROR"
            }
            DeleteCommentResponseContentFailure::NotFound => "DELETE_COMMENT_NOT_FOUND",
            DeleteCommentResponseContentFailure::IncorrectIdFormat { reason: _ } => {
                "DELETE_COMMENT_INCORRECT_ID_FORMAT"
            }
            DeleteCommentResponseContentFailure::Unauthorized { reason: _ } => {
                "DELETE_COMMENT_UNAUTHORIZED"
            }
            DeleteCommentResponseContentFailure::EditingForbidden => {
                "DELETE_COMMENT_DELETING_FORBIDDEN"
            }
        }
    }

    fn reason(&self) -> Option<String> {
        Some(match self {
            DeleteCommentResponseContentFailure::DatabaseError { reason } => {
                if cfg!(debug_assertions) {
                    format!("database error: {}", reason)
                } else {
                    "internal database error".to_string()
                }
            }
            DeleteCommentResponseContentFailure::NotFound => {
                "comment record not found in database".to_string()
            }
            DeleteCommentResponseContentFailure::IncorrectIdFormat { reason } => {
                format!("incorrect value provided for comment ID: {}", reason)
            }
            DeleteCommentResponseContentFailure::Unauthorized { reason } => {
                if cfg!(debug_assertions) {
                    format!("unauthorized error: {}", reason)
                } else {
                    "unauthorized error".to_string()
                }
            }
            DeleteCommentResponseContentFailure::EditingForbidden => {
                String::from("insufficient rights to delete comment")
            }
        })
    }
}
