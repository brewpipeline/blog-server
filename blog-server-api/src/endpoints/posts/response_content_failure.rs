use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentFailure};

pub enum PostsResponseContentFailure {
    DatabaseError { reason: String },
    Unauthorized { reason: String },
    Forbidden,
}

impl ApiResponseContentBase for PostsResponseContentFailure {
    fn status_code(&self) -> &'static StatusCode {
        match self {
            PostsResponseContentFailure::DatabaseError { reason: _ } => {
                &StatusCode::INTERNAL_SERVER_ERROR
            }
            PostsResponseContentFailure::Unauthorized { reason: _ } => &StatusCode::UNAUTHORIZED,
            PostsResponseContentFailure::Forbidden => &StatusCode::FORBIDDEN,
        }
    }
}

impl ApiResponseContentFailure for PostsResponseContentFailure {
    fn identifier(&self) -> &'static str {
        match self {
            PostsResponseContentFailure::DatabaseError { reason: _ } => "POSTS_DATABASE_ERROR",
            PostsResponseContentFailure::Unauthorized { reason: _ } => "POSTS_UNAUTHORIZED",
            PostsResponseContentFailure::Forbidden => "POSTS_FORBIDDEN",
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
            PostsResponseContentFailure::Unauthorized { reason } => {
                if cfg!(debug_assertions) {
                    format!("unauthorized error: {}", reason)
                } else {
                    "unauthorized error".to_string()
                }
            }
            PostsResponseContentFailure::Forbidden => String::from("insufficient rights"),
        })
    }
}
