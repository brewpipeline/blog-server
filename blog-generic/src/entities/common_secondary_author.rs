use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CommonSecondaryAuthor {
    #[validate(length(max = 50), email, non_control_character)]
    pub email: Option<String>,
    #[validate(length(max = 50), phone, non_control_character)]
    pub mobile: Option<String>,
    #[validate(length(max = 100), non_control_character)]
    pub status: Option<String>,
}
