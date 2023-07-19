use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentFailure};

pub enum CreatePostContentFailure {
    DatabaseError { reason: String },
    ValidationError { reason: String },
    AlreadyExists,
    InsertFailed,
}

impl ApiResponseContentBase for CreatePostContentFailure {
    fn status_code(&self) -> &'static StatusCode {
        match self {
            CreatePostContentFailure::DatabaseError { reason: _ } => {
                &StatusCode::INTERNAL_SERVER_ERROR
            }
            CreatePostContentFailure::AlreadyExists => &StatusCode::BAD_REQUEST,
            CreatePostContentFailure::ValidationError { reason: _ } => &StatusCode::BAD_REQUEST,
            CreatePostContentFailure::InsertFailed => &StatusCode::INTERNAL_SERVER_ERROR,
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
            CreatePostContentFailure::AlreadyExists => "CREATE_POST_ALREASY_EXISTS",
            CreatePostContentFailure::InsertFailed => "CREATE_POST_COULD_NOT_FIND_CREATED_POST",
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
            CreatePostContentFailure::InsertFailed => String::from("error while creating new post"),
        })
    }
}
