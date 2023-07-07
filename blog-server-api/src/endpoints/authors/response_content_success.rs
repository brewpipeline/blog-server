use crate::entities::Author;
use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentSuccess};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AuthorsResponseContentSuccess {
    pub(super) authors: Vec<Author>,
    pub(super) total: i64,
    pub(super) offset: i64,
    pub(super) limit: i64,
}

impl ApiResponseContentBase for AuthorsResponseContentSuccess {
    fn status_code(&self) -> &'static StatusCode {
        &StatusCode::OK
    }
}

impl ApiResponseContentSuccess for AuthorsResponseContentSuccess {
    type Data = Self;

    fn identifier(&self) -> &'static str {
        "AUTHORS_OK"
    }

    fn description(&self) -> Option<String> {
        Some("authors list returned".to_string())
    }

    fn data(&self) -> &Self::Data {
        self
    }
}
