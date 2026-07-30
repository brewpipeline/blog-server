use blog_server_api_macros::ApiFailure;

#[derive(ApiFailure)]
pub enum AuthorsResponseContentFailure {
    #[failure(database)]
    DatabaseError { reason: String },
}
