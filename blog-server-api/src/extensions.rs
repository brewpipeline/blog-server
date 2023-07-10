use blog_server_services::impls::{
    create_rbatis_author_service, create_rbatis_comment_service, create_rbatis_post_service,
};
use blog_server_services::traits::author_service::AuthorService;
use blog_server_services::traits::comment_service::CommentService;
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
{
}

struct ExtensionsProvider {
    author_service: Arc<Box<dyn AuthorService>>,
    post_service: Arc<Box<dyn PostService>>,
    comment_service: Arc<Box<dyn CommentService>>,
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

pub fn make_extensions(rbatis: RBatis) -> impl ExtensionsProviderType {
    ExtensionsProvider {
        author_service: Arc::new(create_rbatis_author_service(rbatis.clone())),
        post_service: Arc::new(create_rbatis_post_service(rbatis.clone())),
        comment_service: Arc::new(create_rbatis_comment_service(rbatis.clone())),
    }
}
