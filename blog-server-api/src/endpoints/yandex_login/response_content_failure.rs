use blog_server_api_macros::ApiFailure;

#[derive(ApiFailure)]
pub enum LoginYandexResponseContentFailure {
    #[failure(database)]
    DatabaseError { reason: String },
    #[failure(params)]
    ParamsDecodeError { reason: String },
    #[failure(token)]
    TokenGeneratingError { reason: String },
    #[failure(
        status = BAD_REQUEST,
        reason = "internal yandex vendor error",
        debug_reason = "yandex vendor error: {reason}"
    )]
    YandexError { reason: String },
}
