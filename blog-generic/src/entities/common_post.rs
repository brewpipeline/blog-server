use super::*;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CommonPost {
    #[validate(length(min = 10, max = 75), non_control_character)]
    pub title: String,
    pub published_type: PublishedType,
    #[validate(length(min = 50, max = 255), non_control_character)]
    pub summary: String,
    #[validate(length(min = 50))]
    pub content: Option<String>,
    #[validate(custom(function = "validate_tags"))]
    pub tags: Vec<String>,
    #[validate(length(max = 150), url, non_control_character)]
    pub image_url: Option<String>,
}

fn validate_tags(tags: &Vec<String>) -> Result<(), ValidationError> {
    for tag in tags {
        let chars_count = tag.chars().count();
        if !(3 <= chars_count && chars_count <= 15) {
            return Err(ValidationError::new("wrong_size"));
        }
        if !tag.chars().all(char::is_alphanumeric) {
            return Err(ValidationError::new("is_not_alphanumeric"));
        }
        if !validator::validate_non_control_character(tag) {
            return Err(ValidationError::new("is_non_control_character"));
        }
    }
    Ok(())
}
