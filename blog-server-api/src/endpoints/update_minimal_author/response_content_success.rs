use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentSuccess};

#[derive(Debug, Clone)]
pub struct UpdateMinimalAuthorContentSuccess;

impl Into<UpdateMinimalAuthorContentSuccess> for () {
    fn into(self) -> UpdateMinimalAuthorContentSuccess {
        UpdateMinimalAuthorContentSuccess
    }
}

impl ApiResponseContentBase for UpdateMinimalAuthorContentSuccess {
    fn status_code(&self) -> &'static StatusCode {
        &StatusCode::OK
    }
}

impl ApiResponseContentSuccess for UpdateMinimalAuthorContentSuccess {
    type Data = ();

    fn identifier(&self) -> &'static str {
        "UPDATE_MINIMAL_AUTHOR_SUCCESS"
    }

    fn description(&self) -> Option<String> {
        Some(String::from("minimal author record updated"))
    }

    fn data(&self) -> &Self::Data {
        &()
    }
}
