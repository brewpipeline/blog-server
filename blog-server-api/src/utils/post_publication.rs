use blog_generic::entities::{Post, PublishType};
use blog_generic::events::NewPostPublished;
use blog_server_services::traits::Publish;
use std::sync::Arc;

pub fn announce(
    new_post_service: Arc<dyn Publish<NewPostPublished>>,
    post: &Post,
    was_published: bool,
) {
    if was_published || post.publish_type != PublishType::Published {
        return;
    }

    let new_post_published = NewPostPublished {
        blog_user_id: post.author.id,
        post_sub_url: format!("/post/{}/{}", post.slug, post.id),
    };
    tokio::spawn(async move { new_post_service.publish(new_post_published).await });
}
