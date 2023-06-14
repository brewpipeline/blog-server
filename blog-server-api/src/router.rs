use crate::extensions::ExtensionsProviderType;

use super::endpoints::*;
use hyper::{Body, Method, StatusCode};
use screw_api::json_converter::*;
use screw_core::routing::*;
use screw_core::request::*;
use screw_core::response::*;

async fn not_found_fallback_handler<Extensions>(
    _: router::RoutedRequest<Request<Extensions>>,
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
