pub mod author_service;
pub mod comment_service;
pub mod entity_comment_service;
pub mod entity_post_service;
pub mod post_service;
pub mod social_service;

#[async_trait]
pub trait Publish<T>: Send + Sync {
    async fn publish(&self, event: T) -> ();
}
