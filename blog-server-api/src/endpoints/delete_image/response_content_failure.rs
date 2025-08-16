use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentFailure};

pub enum DeleteImageResponseContentFailure {
    Unauthorized { reason: String },
    Forbidden,
    IoError { reason: String },
    NotFound,
}

impl ApiResponseContentBase for DeleteImageResponseContentFailure {
    fn status_code(&self) -> &'static StatusCode {
        match self {
            DeleteImageResponseContentFailure::Unauthorized { .. } => &StatusCode::UNAUTHORIZED,
            DeleteImageResponseContentFailure::Forbidden => &StatusCode::FORBIDDEN,
            DeleteImageResponseContentFailure::IoError { .. } => &StatusCode::INTERNAL_SERVER_ERROR,
            DeleteImageResponseContentFailure::NotFound => &StatusCode::NOT_FOUND,
        }
    }
}

impl ApiResponseContentFailure for DeleteImageResponseContentFailure {
    fn identifier(&self) -> &'static str {
        match self {
            DeleteImageResponseContentFailure::Unauthorized { .. } => "DELETE_IMAGE_UNAUTHORIZED",
            DeleteImageResponseContentFailure::Forbidden => "DELETE_IMAGE_FORBIDDEN",
            DeleteImageResponseContentFailure::IoError { .. } => "DELETE_IMAGE_IO_ERROR",
            DeleteImageResponseContentFailure::NotFound => "DELETE_IMAGE_NOT_FOUND",
        }
    }

    fn reason(&self) -> Option<String> {
        Some(match self {
            DeleteImageResponseContentFailure::Unauthorized { reason } => {
                if cfg!(debug_assertions) {
                    format!("unauthorized error: {}", reason)
                } else {
                    "unauthorized error".to_string()
                }
            }
            DeleteImageResponseContentFailure::Forbidden => "insufficient rights".to_string(),
            DeleteImageResponseContentFailure::IoError { reason } => {
                if cfg!(debug_assertions) {
                    format!("io error: {}", reason)
                } else {
                    "io error".to_string()
                }
            }
            DeleteImageResponseContentFailure::NotFound => "image not found".to_string(),
        })
    }
}

