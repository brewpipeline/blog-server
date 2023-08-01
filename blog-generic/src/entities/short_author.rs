use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShortAuthor {
    pub slug: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

impl ShortAuthor {
    pub fn image_url(&self) -> String {
        format!(
            "https://api.dicebear.com/6.x/bottts-neutral/svg?seed={}",
            self.slug,
        )
    }
}
