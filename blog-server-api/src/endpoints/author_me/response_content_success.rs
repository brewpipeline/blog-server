use blog_generic::entities::AuthorContainer;
use blog_server_api_macros::ApiSuccess;
use blog_server_services::traits::author_service::Author as ServiceAuthor;

#[derive(Debug, Clone, ApiSuccess)]
#[success(found, description = "auth passed and self author profile returned")]
pub struct AuthorMeResponseContentSuccess {
    container: AuthorContainer,
}

impl From<ServiceAuthor> for AuthorMeResponseContentSuccess {
    fn from(value: ServiceAuthor) -> Self {
        AuthorMeResponseContentSuccess {
            container: AuthorContainer {
                author: value.into(),
            },
        }
    }
}
