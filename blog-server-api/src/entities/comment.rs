use super::*;
use blog_server_services::traits::comment_service::Comment as ServiceComment;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub post_id: i64,
    pub created_at: i64,
    pub content: String,
    pub short_author: ShortAuthor,
}

impl Into<Comment> for ServiceComment {
    fn into(self) -> Comment {
        Comment {
            post_id: self.base.post_id,
            created_at: self.base.created_at,
            content: self.base.content,
            short_author: ShortAuthor {
                slug: self.author_slug,
                first_name: self.author_first_name,
                last_name: self.author_last_name,
            },
        }
    }
}
