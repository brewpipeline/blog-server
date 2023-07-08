use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentFailure};

pub enum PostsResponseContentFailure {
    DatabaseError { reason: String },
}

impl ApiResponseContentBase for PostsResponseContentFailure {
    fn status_code(&self) -> &'static StatusCode {
        match self {
            PostsResponseContentFailure::DatabaseError { reason: _ } => {
                &StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

impl ApiResponseContentFailure for PostsResponseContentFailure {
    fn identifier(&self) -> &'static str {
        match self {
            PostsResponseContentFailure::DatabaseError { reason: _ } => "POSTS_DATABASE_ERROR",
        }
    }

    fn reason(&self) -> Option<String> {
        Some(match self {
            PostsResponseContentFailure::DatabaseError { reason } => {
                if cfg!(debug_assertions) {
                    format!("database error: {}", reason)
                } else {
                    "internal database error".to_string()
                }
            }
        })
    }
}
