use std::marker::PhantomData;
use std::sync::Arc;

use screw_api::request::{ApiRequest, ApiRequestContent, ApiRequestOriginContent};
use screw_api::response::{ApiResponse, ApiResponseContentFailure, ApiResponseContentSuccess};
use screw_components::dyn_fn::{DFnOnce, DFuture};
use screw_core::routing::middleware::Middleware;

use blog_server_api_macros::ApiFailure;
use blog_server_services::traits::author_service::{Author, AuthorService};

use crate::extensions::Resolve;
use crate::utils::auth;

#[derive(Clone, Copy, Debug)]
pub enum AuthPolicy {
    Authenticated,
    NotBlocked,
    Editor,
}

impl AuthPolicy {
    fn check(&self, author: &Author) -> Result<(), AuthRejection> {
        match self {
            AuthPolicy::Authenticated => Ok(()),
            AuthPolicy::NotBlocked => {
                if author.base.blocked == 1 {
                    Err(AuthRejection::AuthorBlocked)
                } else {
                    Ok(())
                }
            }
            AuthPolicy::Editor => {
                if author.base.editor == 0 {
                    Err(AuthRejection::EditorRightsRequired)
                } else {
                    Ok(())
                }
            }
        }
    }
}

#[derive(ApiFailure)]
pub enum AuthRejection {
    #[failure(
        status = UNAUTHORIZED,
        reason = "unauthorized error",
        debug_reason = "unauthorized error: {reason}"
    )]
    Unauthorized { reason: auth::Error },
    #[failure(status = FORBIDDEN, reason = "author is blocked")]
    AuthorBlocked,
    #[failure(status = FORBIDDEN, reason = "insufficient rights")]
    EditorRightsRequired,
}

pub struct AuthApiRequestContent<Content> {
    auth_author_future: DFuture<Result<Author, auth::Error>>,
    inner: Content,
}

impl<Content, Extensions, Failure> ApiRequestContent<Extensions, Failure>
    for AuthApiRequestContent<Content>
where
    Content: ApiRequestContent<Extensions, Failure>,
    Content::Data: Send,
    Extensions: Send + Sync + Resolve<Arc<dyn AuthorService>>,
    Failure: ApiResponseContentFailure,
{
    type Data = Content::Data;

    async fn create(
        origin_content: ApiRequestOriginContent<Self::Data, Extensions>,
    ) -> Result<Self, Failure> {
        let auth_author_future: DFuture<Result<Author, auth::Error>> = Box::pin(auth::author(
            &origin_content.http_parts,
            origin_content.extensions.resolve(),
        ));
        Ok(Self {
            auth_author_future,
            inner: Content::create(origin_content).await?,
        })
    }
}

pub struct AuthorizedApiRequest<Content, Extensions> {
    pub author: Author,
    pub content: Content,
    _p_e: PhantomData<Extensions>,
}

impl<Content, Extensions> From<AuthorizedApiRequest<Content, Extensions>> for (Author, Content) {
    fn from(value: AuthorizedApiRequest<Content, Extensions>) -> Self {
        (value.author, value.content)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct AuthApiMiddleware {
    pub policy: AuthPolicy,
}

impl AuthApiMiddleware {
    pub fn with_policy(policy: AuthPolicy) -> Self {
        Self { policy }
    }
}

impl<Content, Extensions, Success, Failure>
    Middleware<AuthorizedApiRequest<Content, Extensions>, ApiResponse<Success, Failure>>
    for AuthApiMiddleware
where
    Content: ApiRequestContent<Extensions, Failure> + Send + 'static,
    <Content as ApiRequestContent<Extensions, Failure>>::Data: Sync + Send + 'static,
    Extensions: Resolve<Arc<dyn AuthorService>> + Sync + Send + 'static,
    Success: ApiResponseContentSuccess + Send + 'static,
    Failure: ApiResponseContentFailure + From<AuthRejection> + Send + 'static,
{
    type Request = ApiRequest<AuthApiRequestContent<Content>, Extensions>;
    type Response = ApiResponse<Success, Failure>;

    async fn respond(
        &self,
        request: Self::Request,
        next: DFnOnce<AuthorizedApiRequest<Content, Extensions>, ApiResponse<Success, Failure>>,
    ) -> Self::Response {
        let AuthApiRequestContent {
            auth_author_future,
            inner,
        } = request.content;

        let author = match auth_author_future.await {
            Ok(author) => author,
            Err(e) => {
                return ApiResponse::failure(AuthRejection::Unauthorized { reason: e }.into());
            }
        };

        if let Err(rejection) = self.policy.check(&author) {
            return ApiResponse::failure(rejection.into());
        }

        next(AuthorizedApiRequest {
            author,
            content: inner,
            _p_e: PhantomData,
        })
        .await
    }
}

pub struct MaybeAuthorizedApiRequest<Content, Extensions> {
    pub author: Option<Author>,
    pub content: Content,
    _p_e: PhantomData<Extensions>,
}

impl<Content, Extensions> From<MaybeAuthorizedApiRequest<Content, Extensions>>
    for (Option<Author>, Content)
{
    fn from(value: MaybeAuthorizedApiRequest<Content, Extensions>) -> Self {
        (value.author, value.content)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct OptionalAuthApiMiddleware;

impl<Content, Extensions, Success, Failure>
    Middleware<MaybeAuthorizedApiRequest<Content, Extensions>, ApiResponse<Success, Failure>>
    for OptionalAuthApiMiddleware
where
    Content: ApiRequestContent<Extensions, Failure> + Send + 'static,
    <Content as ApiRequestContent<Extensions, Failure>>::Data: Sync + Send + 'static,
    Extensions: Resolve<Arc<dyn AuthorService>> + Sync + Send + 'static,
    Success: ApiResponseContentSuccess + Send + 'static,
    Failure: ApiResponseContentFailure + Send + 'static,
{
    type Request = ApiRequest<AuthApiRequestContent<Content>, Extensions>;
    type Response = ApiResponse<Success, Failure>;

    async fn respond(
        &self,
        request: Self::Request,
        next: DFnOnce<
            MaybeAuthorizedApiRequest<Content, Extensions>,
            ApiResponse<Success, Failure>,
        >,
    ) -> Self::Response {
        let AuthApiRequestContent {
            auth_author_future,
            inner,
        } = request.content;

        next(MaybeAuthorizedApiRequest {
            author: auth_author_future.await.ok(),
            content: inner,
            _p_e: PhantomData,
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use blog_server_services::traits::author_service::BaseAuthor;
    use hyper::StatusCode;
    use screw_api::response::ApiResponseContentBase;

    fn author(editor: u8, blocked: u8) -> Author {
        Author {
            id: 1,
            base: BaseAuthor {
                slug: "slug".to_string(),
                first_name: None,
                middle_name: None,
                last_name: None,
                mobile: None,
                email: None,
                password_hash: None,
                registered_at: 0,
                status: None,
                image_url: None,
                editor,
                blocked,
                yandex_id: None,
                telegram_id: None,
                notification_subscribed: None,
                override_social_data: 0,
            },
        }
    }

    #[test]
    fn authenticated_policy_accepts_blocked_author() {
        assert!(AuthPolicy::Authenticated.check(&author(0, 1)).is_ok());
    }

    #[test]
    fn not_blocked_policy_rejects_blocked_author() {
        let rejection = AuthPolicy::NotBlocked.check(&author(1, 1)).unwrap_err();
        assert_eq!(rejection.status_code(), StatusCode::FORBIDDEN);
        assert_eq!(rejection.identifier(), "AUTHOR_BLOCKED");
    }

    #[test]
    fn not_blocked_policy_accepts_plain_author() {
        assert!(AuthPolicy::NotBlocked.check(&author(0, 0)).is_ok());
    }

    #[test]
    fn editor_policy_rejects_non_editor() {
        let rejection = AuthPolicy::Editor.check(&author(0, 0)).unwrap_err();
        assert_eq!(rejection.status_code(), StatusCode::FORBIDDEN);
        assert_eq!(rejection.identifier(), "EDITOR_RIGHTS_REQUIRED");
    }

    #[test]
    fn editor_policy_accepts_editor() {
        assert!(AuthPolicy::Editor.check(&author(1, 0)).is_ok());
    }

    #[test]
    fn editor_policy_ignores_blocked_flag() {
        assert!(AuthPolicy::Editor.check(&author(1, 1)).is_ok());
    }
}
