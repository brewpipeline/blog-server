use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CommonMinimalAuthor {
    #[validate(custom(function = "validate_slug"))]
    pub slug: String,
    #[validate(length(min = 2, max = 50), non_control_character)]
    pub first_name: Option<String>,
    #[validate(length(min = 2, max = 50), non_control_character)]
    pub last_name: Option<String>,
    #[validate(length(max = 255), url)]
    pub image_url: Option<String>,
}

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
