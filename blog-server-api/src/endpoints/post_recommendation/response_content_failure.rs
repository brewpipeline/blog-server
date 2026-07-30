use blog_server_api_macros::ApiFailure;

#[derive(ApiFailure)]
pub enum PostRecommendationResponseContentFailure {
    #[failure(database)]
    DatabaseError { reason: String },
    #[failure(not_found = "post recommendation")]
    NotFound,
    #[failure(incorrect_id = "post")]
    IncorrectIdFormat { reason: String },
}
