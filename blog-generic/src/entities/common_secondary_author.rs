use serde::{Deserialize, Serialize};
#[cfg(feature = "validator")]
use validator::Validate;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "validator", derive(Validate))]
#[serde(rename_all = "camelCase")]
pub struct CommonSecondaryAuthor {
    #[cfg_attr(
        feature = "validator",
        validate(length(max = 50), email, non_control_character)
    )]
    pub email: Option<String>,
    #[cfg_attr(
        feature = "validator",
        validate(length(max = 50), phone, non_control_character)
    )]
    pub mobile: Option<String>,
    #[cfg_attr(
        feature = "validator",
        validate(length(max = 100), non_control_character)
    )]
    pub status: Option<String>,
}
