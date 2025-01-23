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

/*
#[async_trait]
impl<E> Publish<E> for std::sync::Arc<E>
where
    E: Send + Sync,
{
    async fn publish(&self, event: E) {
        self.publish(event).await
    }
}

pub struct PublishCollection<P, E>(Vec<P>, std::marker::PhantomData<E>)
where
    P: Publish<E>,
    E: Clone + Send + Sync;

impl<P, E> PublishCollection<P, E>
where
    P: Publish<E>,
    E: Clone + Send + Sync,
{
    pub fn new(collection: Vec<P>) -> Self {
        Self(collection, std::marker::PhantomData)
    }
}

#[async_trait]
impl<P, E> Publish<E> for PublishCollection<P, E>
where
    P: Publish<E>,
    E: Clone + Send + Sync,
{
    async fn publish(&self, event: E) {
        for service in self.0.iter() {
            service.publish(event.clone()).await;
        }
    }
}
*/
