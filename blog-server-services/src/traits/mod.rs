pub mod author_service;
pub mod comment_service;
pub mod entity_comment_service;
pub mod entity_post_service;
pub mod post_service;
pub mod social_service;

#[async_trait]
pub trait Publish<E>: Send + Sync {
    async fn publish(&self, event: E) -> ();
}

pub struct PublishCollection<E>(
    Vec<std::sync::Arc<dyn Publish<E>>>,
    std::marker::PhantomData<E>,
)
where
    E: Clone + Send + Sync;

impl<E> PublishCollection<E>
where
    E: Clone + Send + Sync,
{
    pub fn new(collection: Vec<std::sync::Arc<dyn Publish<E>>>) -> Self {
        Self(collection, std::marker::PhantomData)
    }
}

#[async_trait]
impl<E> Publish<E> for PublishCollection<E>
where
    E: Clone + Send + Sync,
{
    async fn publish(&self, event: E) {
        for service in self.0.iter() {
            service.publish(event.clone()).await;
        }
    }
}
