use crate::utils::{string_filter, time_utils, transliteration};
use blog_generic::entities::CommonPost as ECommonPost;
use blog_generic::entities::Tag as ETag;
use screw_components::dyn_result::DResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Tag {
    pub id: u64,
    pub slug: String,
    pub title: String,
}

impl Into<ETag> for Tag {
    fn into(self) -> ETag {
        ETag {
            id: self.id,
            title: self.title,
            slug: self.slug,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BasePost {
    pub author_id: u64,
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub published: u8,
    pub created_at: u64,
    pub content: Option<String>,
}

impl From<(u64, ECommonPost)> for BasePost {
    fn from(value: (u64, ECommonPost)) -> Self {
        BasePost {
            author_id: value.0,
            created_at: time_utils::now_as_secs(),
            slug: {
                let transliterated = transliteration::ru_to_latin_single(
                    value.1.title.clone(),
                    transliteration::TranslitOption::ToLowerCase,
                )
                .transliterated;
                string_filter::remove_non_latin_or_number_chars(&transliterated)
            },
            title: value.1.title,
            summary: value.1.summary,
            published: value.1.published,
            content: value.1.content,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Post {
    pub id: u64,
    #[serde(default)]
    pub tags: Vec<Tag>,
    #[serde(flatten)]
    pub base: BasePost,
}

#[async_trait]
pub trait PostService: Send + Sync {
    async fn posts_count_by_query(&self, query: &String) -> DResult<u64>;
    async fn posts_by_query(&self, query: &String, offset: &u64, limit: &u64)
        -> DResult<Vec<Post>>;
    async fn posts_count_by_author_id(&self, author_id: &u64) -> DResult<u64>;
    async fn posts_by_author_id(
        &self,
        author_id: &u64,
        offset: &u64,
        limit: &u64,
    ) -> DResult<Vec<Post>>;
    async fn posts_count_by_tag_id(&self, tag_id: &u64) -> DResult<u64>;
    async fn posts_by_tag_id(&self, tag_id: &u64, offset: &u64, limit: &u64) -> DResult<Vec<Post>>;
    async fn posts_count(&self) -> DResult<u64>;
    async fn posts(&self, offset: &u64, limit: &u64) -> DResult<Vec<Post>>;
    async fn post_by_id(&self, id: &u64) -> DResult<Option<Post>>;
    async fn create_post(&self, post: &BasePost) -> DResult<u64>;
    async fn update_post_by_id(&self, id: &u64, post: &BasePost) -> DResult<()>;
    async fn delete_post_by_id(&self, id: &u64) -> DResult<()>;

    async fn tag_by_id(&self, id: &u64) -> DResult<Option<Tag>>;
    async fn create_tags(&self, tag_titles: Vec<String>) -> DResult<Vec<Tag>>;
    async fn merge_post_tags(&self, post_id: &u64, tags: Vec<Tag>) -> DResult<()>;
}
