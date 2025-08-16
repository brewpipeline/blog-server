use super::types::{UploadImageError, UploadImageRequest};
use crate::{extensions::Resolve, utils::auth};
use async_trait::async_trait;
use blog_server_services::traits::author_service::AuthorService;
use hyper::{header, Body, StatusCode};
use screw_components::dyn_fn::DFnOnce;
use screw_core::request::Request;
use screw_core::response::Response;
use screw_core::routing::middleware::Middleware;
use screw_core::routing::router::RoutedRequest;
use std::marker::PhantomData;
use std::sync::Arc;

pub struct UploadImageMiddlewareConverter<Extensions> {
    _p: PhantomData<Extensions>,
}

impl<Extensions> Default for UploadImageMiddlewareConverter<Extensions> {
    fn default() -> Self {
        Self { _p: PhantomData }
    }
}

#[async_trait]
impl<Extensions> Middleware<UploadImageRequest, Result<(), UploadImageError>>
    for UploadImageMiddlewareConverter<Extensions>
where
    Extensions: Resolve<Arc<dyn AuthorService>> + Send + Sync + 'static,
{
    type Request = RoutedRequest<Request<Extensions>>;
    type Response = Response;

    async fn respond(
        &self,
        routed_request: RoutedRequest<Request<Extensions>>,
        next: DFnOnce<UploadImageRequest, Result<(), UploadImageError>>,
    ) -> Response {
        use hyper::body::Bytes;

        let (http_parts, http_body) = routed_request.origin.http.into_parts();
        let bytes = hyper::body::to_bytes(http_body)
            .await
            .unwrap_or_else(|_| Bytes::new())
            .to_vec();
        let content_type = http_parts
            .headers
            .get(header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());
        let author = auth::author(&http_parts, routed_request.origin.extensions.resolve()).await;
        let request = UploadImageRequest {
            bytes,
            content_type,
            author,
        };
        let result = next(request).await;
        let http_response = match result {
            Ok(_) => hyper::Response::builder()
                .status(StatusCode::OK)
                .body(Body::empty())
                .unwrap(),
            Err(e) => hyper::Response::builder()
                .status(e.status_code())
                .body(Body::from(e.message()))
                .unwrap(),
        };
        Response { http: http_response }
    }
}
