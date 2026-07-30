use blog_server_api_macros::ApiFailure;

#[derive(ApiFailure)]
pub enum LoginTelegramResponseContentFailure {
    #[failure(database)]
    DatabaseError { reason: String },
    #[failure(params)]
    ParamsDecodeError { reason: String },
    #[failure(token)]
    TokenGeneratingError { reason: String },
    #[failure(
        status = BAD_REQUEST,
        reason = "internal telegram vendor error",
        debug_reason = "telegram vendor error: {reason}"
    )]
    TelegramError { reason: String },
}
