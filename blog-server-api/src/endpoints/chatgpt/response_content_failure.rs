use blog_server_api_macros::ApiFailure;

#[derive(ApiFailure)]
pub enum ChatResponseContentFailure {
    #[failure(database)]
    DatabaseError { reason: String },
    #[failure(params)]
    ParamsDecodeError { reason: String },
    #[failure(
        status = INTERNAL_SERVER_ERROR,
        reason = "internal openai error",
        debug_reason = "openai error: {reason}"
    )]
    OpenAiError { reason: String },
    #[failure(status = TOO_MANY_REQUESTS, reason = "session question limit reached")]
    SessionLimitReached,
}
