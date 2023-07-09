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
        // TODO: refactor next
        let tags: Vec<Tag> = self
            .tags
            .split(";")
            .filter_map(|t| {
                let tag_parts: Vec<&str> = t.split(",").collect();
                if tag_parts.len() == 2 {
                    Some(Tag {
                        title: tag_parts[1].to_owned(),
                        slug: tag_parts[0].to_owned(),
                    })
                } else {
                    None
                }
            })
            .collect();
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
            tags,
        }
    }
}
