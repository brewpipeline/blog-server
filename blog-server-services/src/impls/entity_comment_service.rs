use std::sync::Arc;

use crate::traits::FromAuthored;
use crate::traits::author_service::{Author, AuthorService};
use crate::traits::comment_service::Comment;
use crate::traits::entity_comment_service::EntityCommentService as EntityCommentServiceTrait;
use crate::utils::authors::with_authors;
use blog_generic::entities::Comment as EComment;
use screw_components::dyn_result::DResult;

pub fn create_entity_comment_service(
    author_service: Arc<dyn AuthorService>,
) -> Arc<dyn EntityCommentServiceTrait> {
    Arc::new(EntityCommentService { author_service })
}

impl FromAuthored<Comment> for EComment {
    fn from_authored(comment: Comment, author: Author) -> Self {
        EComment {
            id: comment.id,
            post_id: comment.base.post_id,
            created_at: comment.base.created_at,
            content: if author.base.blocked == 0 && comment.base.published == 1 {
                Some(comment.base.content)
            } else {
                None
            },
            author: author.into(),
        }
    }
}

struct EntityCommentService {
    author_service: Arc<dyn AuthorService>,
}

#[async_trait]
impl EntityCommentServiceTrait for EntityCommentService {
    async fn comments_entities(&self, comments: Vec<Comment>) -> DResult<Vec<EComment>> {
        with_authors(self.author_service.as_ref(), comments).await
    }
}
