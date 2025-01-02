use blog_generic::entities::Tag as ETag;
use blog_generic::entities::{CommonPost as ECommonPost, PublishedType};
use screw_components::dyn_result::DResult;
use serde::{Deserialize, Serialize};

use crate::utils::*;

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
    pub published_type: PublishedType,
    pub created_at: u64,
    pub content: Option<String>,
    pub plain_text_content: Option<String>,
    pub image_url: Option<String>,
}

impl From<(u64, ECommonPost)> for BasePost {
    fn from((author_id, post): (u64, ECommonPost)) -> Self {
        let slug = {
            let transliterated = transliteration::ru_to_latin_single(
                post.title.clone(),
                transliteration::TranslitOption::ToLowerCase,
            )
            .transliterated;
            string_filter::remove_non_latin_or_number_chars(&transliterated)
        };
        let content = post.content.as_ref().map(|c| html::clean(c));
        let plain_text_content = content.as_ref().map(|c| html::to_plain(c));
        BasePost {
            author_id,
            created_at: time_utils::now_as_secs(),
            slug,
            title: post.title,
            summary: post.summary,
            published_type: post.published_type,
            content,
            plain_text_content,
            image_url: post.image_url,
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

pub struct PostsRequest<'q, 'a, 't, 'p, 'o, 'l> {
    pub query: Option<&'q String>,
    pub author_id: Option<&'a u64>,
    pub tag_id: Option<&'t u64>,
    pub published_type: Option<&'p PublishedType>,
    pub offset: &'o u64,
    pub limit: &'l u64,
}

impl<'q, 'a, 't, 'p, 'o, 'l> PostsRequest<'q, 'a, 't, 'p, 'o, 'l> {
    pub fn offset_and_limit(offset: &'o u64, limit: &'l u64) -> Self {
        Self {
            query: None,
            author_id: None,
            tag_id: None,
            published_type: None,
            offset,
            limit,
        }
    }
    pub fn query(mut self, query: Option<&'q String>) -> Self {
        self.query = query;
        self
    }
    pub fn author_id(mut self, author_id: Option<&'a u64>) -> Self {
        self.author_id = author_id;
        self
    }
    pub fn tag_id(mut self, tag_id: Option<&'t u64>) -> Self {
        self.tag_id = tag_id;
        self
    }
    pub fn published_type(mut self, published_type: Option<&'p PublishedType>) -> Self {
        self.published_type = published_type;
        self
    }
}

pub struct PostsResponse {
    pub total_count: u64,
    pub posts: Vec<Post>,
}

#[async_trait]
pub trait PostService: Send + Sync {
    async fn posts<'q, 'a, 't, 'p, 'o, 'l>(
        &self,
        request: PostsRequest<'q, 'a, 't, 'p, 'o, 'l>,
    ) -> DResult<PostsResponse>;

    async fn post_by_id(&self, id: &u64) -> DResult<Option<Post>>;
    async fn create_post(&self, post: &BasePost) -> DResult<u64>;
    async fn update_post_by_id(
        &self,
        id: &u64,
        post: &BasePost,
        update_created_at: &bool,
    ) -> DResult<()>;
    async fn delete_post_by_id(&self, id: &u64) -> DResult<()>;

    async fn tag_by_id(&self, id: &u64) -> DResult<Option<Tag>>;
    async fn create_tags(&self, tag_titles: Vec<String>) -> DResult<Vec<Tag>>;
    async fn merge_post_tags(&self, post_id: &u64, tags: Vec<Tag>) -> DResult<()>;
}
