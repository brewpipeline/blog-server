use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentSuccess};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UploadImageResponseContentSuccessData {
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct UploadImageResponseContentSuccess {
    pub data: UploadImageResponseContentSuccessData,
}

impl ApiResponseContentBase for UploadImageResponseContentSuccess {
    fn status_code(&self) -> &'static StatusCode {
        &StatusCode::OK
    }
}

impl ApiResponseContentSuccess for UploadImageResponseContentSuccess {
    type Data = UploadImageResponseContentSuccessData;

    fn identifier(&self) -> &'static str {
        "UPLOAD_IMAGE_SUCCESS"
    }

    fn description(&self) -> Option<String> {
        Some("image uploaded".to_string())
    }

    fn data(&self) -> &Self::Data {
        &self.data
    }
}

