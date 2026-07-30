use blog_server_api_macros::ApiFailure;

#[derive(ApiFailure)]
pub enum AuthorResponseContentFailure {
    #[failure(database)]
    DatabaseError { reason: String },
    #[failure(not_found = "author")]
    NotFound,
    #[failure(status = BAD_REQUEST, reason = "author slug is empty in request URL")]
    SlugEmpty,
}
