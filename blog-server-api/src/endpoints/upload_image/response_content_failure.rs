use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentFailure};

pub enum UploadImageResponseContentFailure {
    Unauthorized { reason: String },
    Forbidden,
    InvalidData { reason: String },
    IoError { reason: String },
}

impl ApiResponseContentBase for UploadImageResponseContentFailure {
    fn status_code(&self) -> &'static StatusCode {
        match self {
            UploadImageResponseContentFailure::Unauthorized { .. } => &StatusCode::UNAUTHORIZED,
            UploadImageResponseContentFailure::Forbidden => &StatusCode::FORBIDDEN,
            UploadImageResponseContentFailure::InvalidData { .. } => &StatusCode::BAD_REQUEST,
            UploadImageResponseContentFailure::IoError { .. } => &StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl ApiResponseContentFailure for UploadImageResponseContentFailure {
    fn identifier(&self) -> &'static str {
        match self {
            UploadImageResponseContentFailure::Unauthorized { .. } => "UPLOAD_IMAGE_UNAUTHORIZED",
            UploadImageResponseContentFailure::Forbidden => "UPLOAD_IMAGE_FORBIDDEN",
            UploadImageResponseContentFailure::InvalidData { .. } => "UPLOAD_IMAGE_INVALID_DATA",
            UploadImageResponseContentFailure::IoError { .. } => "UPLOAD_IMAGE_IO_ERROR",
        }
    }

    fn reason(&self) -> Option<String> {
        Some(match self {
            UploadImageResponseContentFailure::Unauthorized { reason } => {
                if cfg!(debug_assertions) {
                    format!("unauthorized error: {}", reason)
                } else {
                    "unauthorized error".to_string()
                }
            }
            UploadImageResponseContentFailure::Forbidden => {
                "insufficient rights".to_string()
            }
            UploadImageResponseContentFailure::InvalidData { reason } => {
                format!("invalid data: {}", reason)
            }
            UploadImageResponseContentFailure::IoError { reason } => {
                if cfg!(debug_assertions) {
                    format!("io error: {}", reason)
                } else {
                    "io error".to_string()
                }
            }
        })
    }
}

