use blog_server_api_macros::ApiFailure;

#[derive(ApiFailure)]
pub enum LoginResponseContentFailure {
    #[failure(database)]
    DatabaseError { reason: String },
    #[failure(params)]
    ParamsDecodeError { reason: String },
    #[failure(token)]
    TokenGeneratingError { reason: String },
    #[failure(not_found = "author")]
    NotFound,
    #[failure(status = BAD_REQUEST, reason = "author slug is empty in params")]
    SlugEmpty,
    #[failure(
        status = INTERNAL_SERVER_ERROR,
        reason = "internal password verification error",
        debug_reason = "password verification error: {reason}"
    )]
    PasswordVerificationError { reason: String },
    #[failure(status = FORBIDDEN, reason = "wrong password passed to request")]
    WrongPassword,
    #[failure(status = FORBIDDEN, reason = "lots of login attempts")]
    Blocked,
}
