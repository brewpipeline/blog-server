use hyper::StatusCode;
use screw_api::response::{ApiResponseContentBase, ApiResponseContentSuccess};

#[derive(Debug, Clone)]
pub struct UpdateSecondaryAuthorContentSuccess;

impl Into<UpdateSecondaryAuthorContentSuccess> for () {
    fn into(self) -> UpdateSecondaryAuthorContentSuccess {
        UpdateSecondaryAuthorContentSuccess
    }
}

impl ApiResponseContentBase for UpdateSecondaryAuthorContentSuccess {
    fn status_code(&self) -> &'static StatusCode {
        &StatusCode::OK
    }
}

impl ApiResponseContentSuccess for UpdateSecondaryAuthorContentSuccess {
    type Data = ();

    fn identifier(&self) -> &'static str {
        "UPDATE_SECONDARY_AUTHOR_SUCCESS"
    }

    fn description(&self) -> Option<String> {
        Some(String::from("secondary author record updated"))
    }

    fn data(&self) -> &Self::Data {
        &()
    }
}
