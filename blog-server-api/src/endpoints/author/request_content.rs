use crate::extensions::Resolve;
use blog_server_services::traits::user_service::*;
use screw_api::request::{ApiRequestContent, ApiRequestOriginContent};
use std::sync::Arc;

pub struct AuthorRequestContent {
    pub(super) authorname: Option<String>,
    pub(super) user_service: Arc<Box<dyn UserService>>,
}

impl<Extensions> ApiRequestContent<Extensions> for AuthorRequestContent
where
    Extensions: Resolve<Arc<Box<dyn UserService>>>,
{
    type Data = ();

    fn create(origin_content: ApiRequestOriginContent<Self::Data, Extensions>) -> Self {
        Self {
            authorname: origin_content.path.get("authorname").map(|n| n.to_owned()),
            user_service: origin_content.extensions.resolve(),
        }
    }
}
