use serde::{Deserialize, Serialize};
#[cfg(feature = "validator")]
use validator::{Validate, ValidationError};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "validator", derive(Validate))]
#[serde(rename_all = "camelCase")]
pub struct CommonMinimalAuthor {
    #[cfg_attr(feature = "validator", validate(custom(function = "validate_slug")))]
    pub slug: String,
    #[cfg_attr(
        feature = "validator",
        validate(length(min = 2, max = 50), non_control_character)
    )]
    pub first_name: Option<String>,
    #[cfg_attr(
        feature = "validator",
        validate(length(min = 2, max = 50), non_control_character)
    )]
    pub last_name: Option<String>,
    #[cfg_attr(
        feature = "validator",
        validate(length(max = 255), url, non_control_character)
    )]
    pub image_url: Option<String>,
}

#[cfg(feature = "validator")]
fn validate_slug(slug: &String) -> Result<(), ValidationError> {
    let chars_count = slug.chars().count();
    if !(5 <= chars_count && chars_count <= 30) {
        return Err(ValidationError::new("is_not_in_range_5_30"));
    }
    if !slug.chars().all(|c| char::is_alphanumeric(c) || c == '_') {
        return Err(ValidationError::new(
            "is_not_alphanumeric_or_underscore_character",
        ));
    }
    if !validator::validate_non_control_character(slug) {
        return Err(ValidationError::new("is_not_control_character"));
    }
    Ok(())
}
