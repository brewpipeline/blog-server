use std::sync::Arc;

use crate::traits::FromAuthored;
use crate::traits::author_service::{Author, AuthorService};
use crate::traits::entity_post_service::EntityPostService as EntityPostServiceTrait;
use crate::traits::post_service::{BasePost, Post};
use crate::utils::authors::with_authors;
use crate::utils::image_signer::{ImageVariant, processed_image_urls};
use blog_generic::entities::{Post as EPost, PublishType};
use screw_components::dyn_result::DResult;

pub fn create_entity_post_service(
    author_service: Arc<dyn AuthorService>,
) -> Arc<dyn EntityPostServiceTrait> {
    Arc::new(EntityPostService { author_service })
}

impl FromAuthored<Post> for EPost {
    fn from_authored(post: Post, author: Author) -> Self {
        let noindex = post.base.publish_type != PublishType::Published
            || post
                .base
                .lang
                .as_deref()
                .is_some_and(|l| BasePost::current_lang().is_some_and(|cl| l != cl));
        let processed_image_urls = {
            let cover: Vec<(&str, ImageVariant)> = post
                .base
                .image_url
                .as_deref()
                .map(|u| (u, ImageVariant::Medium))
                .into_iter()
                .collect();
            processed_image_urls(&cover, post.base.content.as_deref())
        };
        EPost {
            id: post.id,
            title: post.base.title,
            slug: post.base.slug,
            summary: post.base.summary,
            publish_type: post.base.publish_type,
            recommended: post.recommended != 0,
            created_at: post.base.created_at,
            content: post.base.content,
            author: author.into(),
            tags: post.tags.into_iter().map(Into::into).collect(),
            image_url: post.base.image_url,
            processed_image_urls,
            noindex,
        }
    }
}

struct EntityPostService {
    author_service: Arc<dyn AuthorService>,
}

#[async_trait]
impl EntityPostServiceTrait for EntityPostService {
    async fn posts_entities(&self, posts: Vec<Post>) -> DResult<Vec<EPost>> {
        with_authors(self.author_service.as_ref(), posts).await
    }
}
