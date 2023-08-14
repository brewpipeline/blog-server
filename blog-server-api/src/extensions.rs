use blog_server_services::impls::{
    create_entity_comment_service, create_entity_post_service, create_rbatis_author_service,
    create_rbatis_comment_service, create_rbatis_post_service,
};
use blog_server_services::traits::author_service::AuthorService;
use blog_server_services::traits::comment_service::CommentService;
use blog_server_services::traits::entity_comment_service::EntityCommentService;
use blog_server_services::traits::entity_post_service::EntityPostService;
use blog_server_services::traits::post_service::PostService;
use rbatis::rbatis::RBatis;
use std::sync::Arc;

pub trait Resolve<T>: Send + Sync {
    fn resolve(&self) -> T;
}

pub trait ExtensionsProviderType:
    Resolve<Arc<Box<dyn AuthorService>>>
    + Resolve<Arc<Box<dyn PostService>>>
    + Resolve<Arc<Box<dyn CommentService>>>
    + Resolve<Arc<Box<dyn EntityCommentService>>>
    + Resolve<Arc<Box<dyn EntityPostService>>>
{
}

struct ExtensionsProvider {
    author_service: Arc<Box<dyn AuthorService>>,
    post_service: Arc<Box<dyn PostService>>,
    comment_service: Arc<Box<dyn CommentService>>,
    entity_comment_service: Arc<Box<dyn EntityCommentService>>,
    entity_post_service: Arc<Box<dyn EntityPostService>>,
}

impl ExtensionsProviderType for ExtensionsProvider {}

impl Resolve<Arc<Box<dyn AuthorService>>> for ExtensionsProvider {
    fn resolve(&self) -> Arc<Box<dyn AuthorService>> {
        self.author_service.clone()
    }
}

impl Resolve<Arc<Box<dyn PostService>>> for ExtensionsProvider {
    fn resolve(&self) -> Arc<Box<dyn PostService>> {
        self.post_service.clone()
    }
}

impl Resolve<Arc<Box<dyn CommentService>>> for ExtensionsProvider {
    fn resolve(&self) -> Arc<Box<dyn CommentService>> {
        self.comment_service.clone()
    }
}

impl Resolve<Arc<Box<dyn EntityCommentService>>> for ExtensionsProvider {
    fn resolve(&self) -> Arc<Box<dyn EntityCommentService>> {
        self.entity_comment_service.clone()
    }
}

impl Resolve<Arc<Box<dyn EntityPostService>>> for ExtensionsProvider {
    fn resolve(&self) -> Arc<Box<dyn EntityPostService>> {
        self.entity_post_service.clone()
    }
}

pub fn make_extensions(rbatis: RBatis) -> impl ExtensionsProviderType {
    let authors_service = Arc::new(create_rbatis_author_service(rbatis.clone()));
    ExtensionsProvider {
        author_service: authors_service.clone(),
        post_service: Arc::new(create_rbatis_post_service(rbatis.clone())),
        comment_service: Arc::new(create_rbatis_comment_service(rbatis.clone())),
        entity_comment_service: Arc::new(create_entity_comment_service(authors_service.clone())),
        entity_post_service: Arc::new(create_entity_post_service(authors_service.clone())),
    }
}
