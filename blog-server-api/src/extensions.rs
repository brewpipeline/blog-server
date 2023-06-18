use blog_server_services::instances::create_rbatis_author_service;
use blog_server_services::traits::author_service::AuthorService;
use rbatis::rbatis::RBatis;
use std::sync::Arc;

pub trait Resolve<T>: Send + Sync {
    fn resolve(&self) -> T;
}

pub trait ExtensionsProviderType: Resolve<Arc<Box<dyn AuthorService>>> {}

struct ExtensionsProvider {
    author_service: Arc<Box<dyn AuthorService>>,
}

impl ExtensionsProviderType for ExtensionsProvider {}

impl Resolve<Arc<Box<dyn AuthorService>>> for ExtensionsProvider {
    fn resolve(&self) -> Arc<Box<dyn AuthorService>> {
        self.author_service.clone()
    }
}

pub fn make_extensions(rbatis: RBatis) -> impl ExtensionsProviderType {
    ExtensionsProvider {
        author_service: Arc::new(create_rbatis_author_service(rbatis)),
    }
}
