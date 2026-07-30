use blog_server_api_macros::ApiSuccess;

#[derive(Debug, Clone, ApiSuccess)]
#[success(ok, description = "post record deleted")]
pub struct DeletePostResponseContentSuccess;
