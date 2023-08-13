use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::traits::author_service::{Author, AuthorService};
use crate::traits::entity_post_service::EntityPostService as EntityPostServiceTrait;
use crate::traits::post_service::Post;
use blog_generic::entities::{Post as EPost, Tag as ETag};
use screw_components::dyn_result::{DError, DResult};

pub fn create_entity_post_service(
    author_service: Arc<Box<dyn AuthorService>>,
) -> Box<dyn EntityPostServiceTrait> {
    Box::new(EntityPostService { author_service })
}

struct EPostBuilder(Post, Author);

impl Into<EPost> for EPostBuilder {
    fn into(self) -> EPost {
        EPost {
            id: self.0.id,
            title: self.0.base.title,
            slug: self.0.base.slug,
            summary: self.0.base.summary,
            created_at: self.0.base.created_at,
            content: self.0.base.content,
            author: self.1.into(),
            tags: self
                .0
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

struct EntityPostService {
    author_service: Arc<Box<dyn AuthorService>>,
}

#[async_trait]
impl EntityPostServiceTrait for EntityPostService {
    async fn posts_entities(&self, posts: Vec<Post>) -> DResult<Vec<EPost>> {
        let authors_ids = posts
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

        let mut posts_entities: Vec<EPost> = vec![];
        for post in posts {
            let post_author = authors_map
                .get(&post.base.author_id)
                .cloned()
                .ok_or::<DError>("wrong authors map".into())?;
            let post_entity = EPostBuilder(post, post_author).into();
            posts_entities.push(post_entity);
        }
        Ok(posts_entities)
    }
}
