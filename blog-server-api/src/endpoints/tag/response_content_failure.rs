use blog_server_api_macros::ApiFailure;

#[derive(ApiFailure)]
pub enum TagResponseContentFailure {
    #[failure(database)]
    DatabaseError { reason: String },
    #[failure(not_found = "tag")]
    NotFound,
    #[failure(incorrect_id = "tag")]
    IncorrectIdFormat { reason: String },
}
