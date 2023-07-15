use super::*;
use blog_server_services::traits::post_service::Post as ServicePost;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub created_at: u64,
    pub content: Option<String>,
    pub short_author: ShortAuthor,
    pub tags: Vec<Tag>,
}

impl Into<Post> for ServicePost {
    fn into(self) -> Post {
        Post {
            id: self.id,
            title: self.base.title,
            slug: self.base.slug,
            summary: self.base.summary,
            created_at: self.base.created_at,
            content: self.base.content,
            short_author: ShortAuthor {
                slug: self.author_slug,
                first_name: self.author_first_name,
                last_name: self.author_last_name,
            },
            tags: self
                .tags
                .into_iter()
                .map(|v| Tag {
                    id: v.id,
                    title: v.title,
                    slug: v.slug,
                })
                .collect(),
        }
    }
}
