use blog_generic::entities::AuthorsContainer;
use blog_server_api_macros::ApiSuccess;

#[derive(Debug, Clone, ApiSuccess)]
#[success(found, description = "authors list returned")]
pub struct AuthorsResponseContentSuccess {
    pub(super) container: AuthorsContainer,
}
