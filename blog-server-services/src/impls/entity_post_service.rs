use std::sync::Arc;

use crate::traits::author_service::{Author, AuthorService};
use crate::traits::entity_post_service::EntityPostService as EntityPostServiceTrait;
use crate::traits::post_service::{BasePost, Post, PostService, Tag};
use crate::utils::authors::{authors_by_ids, authors_ids};
use crate::utils::image_signer::{
    ImageVariant, cached_image_urls, processed_image_urls, replace_with_cached,
};
use blog_generic::entities::{Post as EPost, PublishType};
use screw_components::dyn_result::{DError, DResult};
use std::collections::{HashMap, HashSet};

pub fn create_entity_post_service(
    author_service: Arc<dyn AuthorService>,
    post_service: Arc<dyn PostService>,
) -> Arc<dyn EntityPostServiceTrait> {
    Arc::new(EntityPostService {
        author_service,
        post_service,
    })
}

fn post_image_urls(post: &Post) -> HashMap<String, String> {
    let cover: Vec<(&str, ImageVariant)> = post
        .base
        .image_url
        .as_deref()
        .map(|u| (u, ImageVariant::Medium))
        .into_iter()
        .collect();
    processed_image_urls(&cover, post.base.content.as_deref())
}

fn post_entity(
    post: Post,
    tags: Vec<Tag>,
    author: Author,
    processed_image_urls: HashMap<String, String>,
) -> EPost {
    let noindex = post.base.publish_type != PublishType::Published
        || post
            .base
            .lang
            .as_deref()
            .is_some_and(|l| BasePost::current_lang().is_some_and(|cl| l != cl));
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
        tags: tags.into_iter().map(Into::into).collect(),
        image_url: post.base.image_url,
        processed_image_urls,
        noindex,
    }
}

struct EntityPostService {
    author_service: Arc<dyn AuthorService>,
    post_service: Arc<dyn PostService>,
}

#[async_trait]
impl EntityPostServiceTrait for EntityPostService {
    async fn posts_entities(&self, posts: Vec<Post>) -> DResult<Vec<EPost>> {
        if posts.is_empty() {
            return Ok(vec![]);
        }

        let post_ids: HashSet<u64> = posts.iter().map(|post| post.id).collect();
        let authors_ids = authors_ids(&posts);
        let posts_image_urls: Vec<HashMap<String, String>> =
            posts.iter().map(post_image_urls).collect();

        let (mut tags, authors, cached_image_urls) = tokio::try_join!(
            self.post_service.tags_by_post_ids(&post_ids),
            authors_by_ids(self.author_service.as_ref(), &authors_ids),
            async { DResult::Ok(cached_image_urls(posts_image_urls.iter()).await) },
        )?;

        posts
            .into_iter()
            .zip(posts_image_urls)
            .map(|(post, mut image_urls)| {
                let author = authors
                    .get(&post.base.author_id)
                    .cloned()
                    .ok_or::<DError>("wrong authors map".into())?;
                let tags = tags.remove(&post.id).unwrap_or_default();
                replace_with_cached(&cached_image_urls, &mut image_urls);
                Ok(post_entity(post, tags, author, image_urls))
            })
            .collect()
    }
}
