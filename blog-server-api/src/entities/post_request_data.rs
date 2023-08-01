use blog_server_services::{
    traits::post_service::BasePost,
    utils::{string_filter, transliteration},
};
use serde::{Deserialize, Serialize};

use crate::utils::time_utils;

//TODO move from entities to requestDTOs if more of these emerge
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostRequestData {
    pub title: String,
    pub summary: String,
    pub published: u8,
    pub content: Option<String>,
    pub tags: Vec<String>,
}

impl PostRequestData {
    pub fn into(self, author_id: u64) -> BasePost {
        BasePost {
            author_id,
            created_at: time_utils::now_as_secs(),
            slug: {
                let transliterated = transliteration::ru_to_latin_single(
                    self.title.clone(),
                    transliteration::TranslitOption::ToLowerCase,
                )
                .transliterated;
                string_filter::remove_non_latin_or_number_chars(&transliterated)
            },
            title: self.title,
            summary: self.summary,
            published: self.published,
            content: self.content,
        }
    }
}
