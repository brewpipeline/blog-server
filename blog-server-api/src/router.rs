use crate::extensions::ExtensionsProviderType;

use super::endpoints::*;
use hyper::{Body, Method, StatusCode};
use screw_api::json_converter::{JsonApiRequestConverter, JsonApiResponseConverter};
use screw_core::routing::request::DirectedRequest;
use screw_core::routing::*;
use screw_core::{Request, Response};

async fn not_found_fallback_handler<Extensions>(
    _: DirectedRequest<Request<Extensions>>,
) -> Response {
    Response {
        http: hyper::Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap(),
    }
}

pub fn make_router<Extensions: ExtensionsProviderType>(
) -> router::second::Router<Request<Extensions>, Response> {
    router::first::Router::with_fallback_handler(not_found_fallback_handler).and_routes(|r| {
        r.scoped_convertable(
            "/api",
            routes::Converters {
                request_converter: JsonApiRequestConverter,
                response_converter: JsonApiResponseConverter {
                    pretty_printed: false,
                },
            },
            |r| {
                r.route(
                    route::first::Route::with_method(&Method::GET)
                        .and_path("/author")
                        .and_handler(author::http_handler),
                )
            },
        )
    })
}
