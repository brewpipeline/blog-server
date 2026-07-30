use blog_server_api_macros::ApiFailure;

#[derive(ApiFailure)]
pub enum CommentsResponseContentFailure {
    #[failure(database)]
    DatabaseError { reason: String },
    #[failure(incorrect_id = "post")]
    IncorrectIdFormat { reason: String },
}
