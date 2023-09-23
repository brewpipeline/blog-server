use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentFailure};

pub enum CreatePostContentFailure {
    DatabaseError { reason: String },
    ValidationError { reason: String },
    InsertFailed,
    Unauthorized { reason: String },
    CreatingForbidden,
}

impl ApiResponseContentBase for CreatePostContentFailure {
    fn status_code(&self) -> &'static StatusCode {
        match self {
            CreatePostContentFailure::DatabaseError { reason: _ } => {
                &StatusCode::INTERNAL_SERVER_ERROR
            }
            CreatePostContentFailure::ValidationError { reason: _ } => &StatusCode::BAD_REQUEST,
            CreatePostContentFailure::Unauthorized { reason: _ } => &StatusCode::UNAUTHORIZED,
            CreatePostContentFailure::InsertFailed => &StatusCode::INTERNAL_SERVER_ERROR,
            CreatePostContentFailure::CreatingForbidden => &StatusCode::FORBIDDEN,
        }
    }
}

impl ApiResponseContentFailure for CreatePostContentFailure {
    fn identifier(&self) -> &'static str {
        match self {
            CreatePostContentFailure::DatabaseError { reason: _ } => "CREATE_POST_DATABASE_ERROR",
            CreatePostContentFailure::ValidationError { reason: _ } => {
                "CREATE_POST_VALIDATION_ERROR"
            }
            CreatePostContentFailure::InsertFailed => "CREATE_POST_COULD_NOT_FIND_CREATED_POST",
            CreatePostContentFailure::Unauthorized { reason: _ } => "CREATE_POST_UNAUTHORIZED",
            CreatePostContentFailure::CreatingForbidden => "CREATE_POST_CREATING_FORBIDDEN",
        }
    }

    fn reason(&self) -> Option<String> {
        Some(match self {
            CreatePostContentFailure::DatabaseError { reason } => {
                if cfg!(debug_assertions) {
                    format!("database error: {}", reason)
                } else {
                    "internal database error".to_string()
                }
            }
            CreatePostContentFailure::Unauthorized { reason } => {
                if cfg!(debug_assertions) {
                    format!("unauthorized error: {}", reason)
                } else {
                    "unauthorized error".to_string()
                }
            }
            CreatePostContentFailure::ValidationError { reason } => {
                format!("validation error: {}", reason)
            }
            CreatePostContentFailure::InsertFailed => String::from("error while creating new post"),
            CreatePostContentFailure::CreatingForbidden => {
                String::from("insufficient rights to create post")
            }
        })
    }
}
