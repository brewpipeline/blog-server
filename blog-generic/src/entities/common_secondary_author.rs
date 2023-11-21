use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CommonSecondaryAuthor {
    #[validate(email)]
    pub email: Option<String>,
    #[validate(phone)]
    pub mobile: Option<String>,
    #[validate(length(max = 255), non_control_character)]
    pub status: Option<String>,
}
