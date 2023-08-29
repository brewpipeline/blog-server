use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub id: u64,
    pub slug: String,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub mobile: Option<String>,
    pub email: Option<String>,
    pub registered_at: u64,
    pub status: Option<String>,
    pub image_url: Option<String>,
}

impl Author {
    pub fn default_image_url(&self) -> String {
        format!(
            "https://api.dicebear.com/7.x/shapes/svg?seed={seed}",
            seed = self.slug,
        )
    }

    pub fn image_url(&self) -> String {
        self.image_url
            .clone()
            .unwrap_or_else(|| self.default_image_url())
    }
}
