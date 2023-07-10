use super::*;
use blog_server_services::traits::post_service::Post as ServicePost;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub created_at: i64,
    pub content: Option<String>,
    pub short_author: ShortAuthor,
    pub tags: Vec<Tag>,
}

impl Into<Post> for ServicePost {
    fn into(self) -> Post {
        Post {
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
                    title: v.title,
                    //TODO: Change to ID
                    slug: v.id.to_string(),
                })
                .collect(),
        }
    }
}
