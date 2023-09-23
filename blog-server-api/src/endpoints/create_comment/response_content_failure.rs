use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentFailure};

pub enum CreateCommentContentFailure {
    DatabaseError { reason: String },
    ValidationError { reason: String },
    InsertFailed,
    Unauthorized { reason: String },
    CreatingForbidden,
}

impl ApiResponseContentBase for CreateCommentContentFailure {
    fn status_code(&self) -> &'static StatusCode {
        match self {
            CreateCommentContentFailure::DatabaseError { reason: _ } => {
                &StatusCode::INTERNAL_SERVER_ERROR
            }
            CreateCommentContentFailure::ValidationError { reason: _ } => &StatusCode::BAD_REQUEST,
            CreateCommentContentFailure::Unauthorized { reason: _ } => &StatusCode::UNAUTHORIZED,
            CreateCommentContentFailure::InsertFailed => &StatusCode::INTERNAL_SERVER_ERROR,
            CreateCommentContentFailure::CreatingForbidden => &StatusCode::FORBIDDEN,
        }
    }
}

impl ApiResponseContentFailure for CreateCommentContentFailure {
    fn identifier(&self) -> &'static str {
        match self {
            CreateCommentContentFailure::DatabaseError { reason: _ } => {
                "CREATE_COMMENT_DATABASE_ERROR"
            }
            CreateCommentContentFailure::ValidationError { reason: _ } => {
                "CREATE_COMMENT_VALIDATION_ERROR"
            }
            CreateCommentContentFailure::InsertFailed => {
                "CREATE_COMMENT_COULD_NOT_FIND_CREATED_COMMENT"
            }
            CreateCommentContentFailure::Unauthorized { reason: _ } => {
                "CREATE_COMMENT_UNAUTHORIZED"
            }
            CreateCommentContentFailure::CreatingForbidden => "CREATE_COMMENT_CREATING_FORBIDDEN",
        }
    }

    fn reason(&self) -> Option<String> {
        Some(match self {
            CreateCommentContentFailure::DatabaseError { reason } => {
                if cfg!(debug_assertions) {
                    format!("database error: {}", reason)
                } else {
                    "internal database error".to_string()
                }
            }
            CreateCommentContentFailure::Unauthorized { reason } => {
                if cfg!(debug_assertions) {
                    format!("unauthorized error: {}", reason)
                } else {
                    "unauthorized error".to_string()
                }
            }
            CreateCommentContentFailure::ValidationError { reason } => {
                format!("validation error: {}", reason)
            }
            CreateCommentContentFailure::InsertFailed => {
                String::from("error while creating new comment")
            }
            CreateCommentContentFailure::CreatingForbidden => {
                String::from("insufficient rights to create comment")
            }
        })
    }
}
