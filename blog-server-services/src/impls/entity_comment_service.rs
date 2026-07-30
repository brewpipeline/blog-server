use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::traits::author_service::{Author, AuthorService};
use crate::traits::comment_service::Comment;
use crate::traits::entity_comment_service::EntityCommentService as EntityCommentServiceTrait;
use blog_generic::entities::Comment as EComment;
use screw_components::dyn_result::{DError, DResult};

pub fn create_entity_comment_service(
    author_service: Arc<dyn AuthorService>,
) -> Arc<dyn EntityCommentServiceTrait> {
    Arc::new(EntityCommentService { author_service })
}

struct ECommentBuilder(Comment, Author);

impl From<ECommentBuilder> for EComment {
    fn from(value: ECommentBuilder) -> Self {
        EComment {
            id: value.0.id,
            post_id: value.0.base.post_id,
            created_at: value.0.base.created_at,
            content: if value.1.base.blocked == 0 && value.0.base.published == 1 {
                Some(value.0.base.content)
            } else {
                None
            },
            author: value.1.into(),
        }
    }
}

struct EntityCommentService {
    author_service: Arc<dyn AuthorService>,
}

#[async_trait]
impl EntityCommentServiceTrait for EntityCommentService {
    async fn comments_entities(&self, comments: Vec<Comment>) -> DResult<Vec<EComment>> {
        let authors_ids = comments
            .iter()
            .map(|c| c.base.author_id)
            .collect::<HashSet<_>>();
        let authors_map = self
            .author_service
            .authors_by_ids(&authors_ids.into())
            .await?
            .into_iter()
            .map(|a| (a.id, a))
            .collect::<HashMap<_, _>>();

        let mut comments_entities: Vec<EComment> = vec![];
        for comment in comments {
            let comment_author = authors_map
                .get(&comment.base.author_id)
                .cloned()
                .ok_or::<DError>("wrong authors map".into())?;
            let comment_entity = ECommentBuilder(comment, comment_author).into();
            comments_entities.push(comment_entity);
        }
        Ok(comments_entities)
    }
}
