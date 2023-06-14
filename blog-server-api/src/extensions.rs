use std::sync::Arc;
use blog_server_services::instances::create_rbatis_user_service;
use blog_server_services::traits::user_service::UserService;
use rbatis::rbatis::RBatis;

pub trait Resolve<T>: Send + Sync {
    fn resolve(&self) -> T;
}

pub trait ExtensionsProviderType: Resolve<Arc<Box<dyn UserService>>> {}

struct ExtensionsProvider {
    user_service: Arc<Box<dyn UserService>>,
}

impl ExtensionsProviderType for ExtensionsProvider {}

impl Resolve<Arc<Box<dyn UserService>>> for ExtensionsProvider {
    fn resolve(&self) -> Arc<Box<dyn UserService>> {
        self.user_service.clone()
    }
}

pub fn make_extensions(rbatis: RBatis) -> impl ExtensionsProviderType {
    ExtensionsProvider {
        user_service: Arc::new(create_rbatis_user_service(rbatis)),
    }
}
