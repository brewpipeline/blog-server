use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentFailure};

pub enum CreatePostContentFailure {
    DatabaseError { reason: String },
    ValidationError { reason: String },
    AlreadyExists,
}

impl ApiResponseContentBase for CreatePostContentFailure {
    fn status_code(&self) -> &'static StatusCode {
        match self {
            CreatePostContentFailure::DatabaseError { reason: _ } => {
                &StatusCode::INTERNAL_SERVER_ERROR
            }
            CreatePostContentFailure::AlreadyExists => &StatusCode::BAD_REQUEST,
            CreatePostContentFailure::ValidationError { reason: _ } => &StatusCode::BAD_REQUEST,
        }
    }
}

impl ApiResponseContentFailure for CreatePostContentFailure {
    fn identifier(&self) -> &'static str {
        match self {
            CreatePostContentFailure::DatabaseError { reason: _ } => "POST_DATABASE_ERROR",
            CreatePostContentFailure::ValidationError { reason: _ } => "POST_VALIDATION_ERROR",
            CreatePostContentFailure::AlreadyExists => "POST_ALREASY_EXISTS",
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
            CreatePostContentFailure::ValidationError { reason } => {
                format!("validation error: {}", reason)
            }
            CreatePostContentFailure::AlreadyExists => {
                String::from("post with specified ID already exists")
            }
        })
    }
}
