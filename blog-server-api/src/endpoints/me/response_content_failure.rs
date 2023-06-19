use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentFailure};

pub enum MeResponseContentFailure {
    Unauthorized { reason: String },
}

impl ApiResponseContentBase for MeResponseContentFailure {
    fn status_code(&self) -> &'static StatusCode {
        match self {
            MeResponseContentFailure::Unauthorized { reason: _ } => &StatusCode::UNAUTHORIZED,
        }
    }
}

impl ApiResponseContentFailure for MeResponseContentFailure {
    fn identifier(&self) -> &'static str {
        match self {
            MeResponseContentFailure::Unauthorized { reason: _ } => "ME_UNAUTHORIZED",
        }
    }

    fn reason(&self) -> Option<String> {
        Some(match self {
            MeResponseContentFailure::Unauthorized { reason } => {
                if cfg!(debug_assertions) {
                    format!("unauthorized error: {}", reason)
                } else {
                    "unauthorized error".to_string()
                }
            }
        })
    }
}
