use blog_server_api_macros::ApiSuccess;

#[derive(Debug, Clone, ApiSuccess)]
#[success(ok, description = "author override social data state changed")]
pub struct AuthorOverrideSocialDataResponseContentSuccess;
