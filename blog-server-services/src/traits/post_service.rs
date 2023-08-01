use crate::utils::{time_utils, transliteration};
use blog_generic::entities::{
    CommonPost as ECommonPost, Post as EPost, ShortAuthor as EShortAuthor, Tag as ETag,
};
use screw_components::dyn_result::DResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Tag {
    pub id: u64,
    pub slug: String,
    pub title: String,
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
            slug: transliteration::ru_to_latin_single(value.1.title.clone().to_lowercase())
                .transliterated,
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
    pub author_slug: String,
    pub author_first_name: Option<String>,
    pub author_last_name: Option<String>,
    #[serde(default)]
    pub tags: Vec<Tag>,
    #[serde(flatten)]
    pub base: BasePost,
}

impl Into<EPost> for Post {
    fn into(self) -> EPost {
        EPost {
            id: self.id,
            title: self.base.title,
            slug: self.base.slug,
            summary: self.base.summary,
            created_at: self.base.created_at,
            content: self.base.content,
            short_author: EShortAuthor {
                slug: self.author_slug,
                first_name: self.author_first_name,
                last_name: self.author_last_name,
            },
            tags: self
                .tags
                .into_iter()
                .map(|v| ETag {
                    id: v.id,
                    title: v.title,
                    slug: v.slug,
                })
                .collect(),
        }
    }
}

#[async_trait]
pub trait PostService: Send + Sync {
    async fn posts_count_by_query(&self, query: &String) -> DResult<u64>;
    async fn posts_by_query(&self, query: &String, offset: &u64, limit: &u64)
        -> DResult<Vec<Post>>;
    async fn posts_count(&self) -> DResult<u64>;
    async fn posts(&self, offset: &u64, limit: &u64) -> DResult<Vec<Post>>;
    async fn post_by_id(&self, id: &u64) -> DResult<Option<Post>>;
    async fn create_post(&self, post: &BasePost) -> DResult<u64>;
    async fn update_post(&self, post_id: &u64, post: &BasePost) -> DResult<()>;

    async fn create_tags(&self, tag_titles: Vec<String>) -> DResult<Vec<Tag>>;
    async fn merge_post_tags(&self, post_id: &u64, tags: Vec<Tag>) -> DResult<()>;
}
