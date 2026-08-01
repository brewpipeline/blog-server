use blog_server_api_macros::ApiSuccess;

#[derive(Debug, Clone, ApiSuccess)]
#[success(ok, description = "minimal author record updated")]
pub struct UpdateMinimalAuthorContentSuccess;
