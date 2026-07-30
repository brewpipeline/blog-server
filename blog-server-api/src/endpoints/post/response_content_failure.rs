use blog_server_api_macros::ApiFailure;

#[derive(ApiFailure)]
pub enum PostResponseContentFailure {
    #[failure(database)]
    DatabaseError { reason: String },
    #[failure(not_found = "post")]
    NotFound,
    #[failure(incorrect_id = "post")]
    IncorrectIdFormat { reason: String },
}
