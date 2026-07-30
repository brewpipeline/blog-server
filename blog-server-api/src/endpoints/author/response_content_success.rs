use blog_generic::entities::AuthorContainer;
use blog_server_api_macros::ApiSuccess;
use blog_server_services::traits::author_service::Author as ServiceAuthor;

#[derive(Debug, Clone, ApiSuccess)]
#[success(found, description = "author record found")]
pub struct AuthorResponseContentSuccess {
    pub(super) container: AuthorContainer,
}

impl From<ServiceAuthor> for AuthorResponseContentSuccess {
    fn from(value: ServiceAuthor) -> Self {
        AuthorResponseContentSuccess {
            container: AuthorContainer {
                author: value.into(),
            },
        }
    }
}
