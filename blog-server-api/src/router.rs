use super::endpoints::*;
use super::extensions::*;
use screw_api::json_converter::*;
use screw_api::request::*;
use screw_api::response::*;
use screw_core::request::*;
use screw_core::response::*;
use screw_core::routing::*;

async fn not_found_fallback_handler<Extensions>(
    _: router::RoutedRequest<Request<Extensions>>,
) -> Response {
    Response {
        http: hyper::Response::builder()
            .status(hyper::StatusCode::NOT_FOUND)
            .body(hyper::Body::empty())
            .unwrap(),
    }
}

struct NotFoundResponseContentFailure;

impl ApiResponseContentBase for NotFoundResponseContentFailure {
    fn status_code(&self) -> &'static hyper::StatusCode {
        &hyper::StatusCode::NOT_FOUND
    }
}

impl ApiResponseContentFailure for NotFoundResponseContentFailure {
    fn identifier(&self) -> &'static str {
        "NOT_FOUND"
    }
    fn reason(&self) -> Option<String> {
        Some("route not found".to_string())
    }
}

async fn api_not_found_fallback_handler<Extensions>(
    _: ApiRequest<(), Extensions>,
) -> ApiResponse<std::convert::Infallible, NotFoundResponseContentFailure> {
    ApiResponse::failure(NotFoundResponseContentFailure)
}

pub fn make_router<Extensions: ExtensionsProviderType>(
) -> router::second::Router<Request<Extensions>, Response> {
    router::first::Router::with_fallback_handler(not_found_fallback_handler).and_routes(|r| {
        r.scoped_convertable(
            "/api",
            routes::Converters {
                request_converter: JsonApiRequestConverter,
                response_converter: JsonApiResponseConverter {
                    pretty_printed: cfg!(debug_assertions),
                },
            },
            |r| {
                r.route(
                    route::first::Route::with_method(&hyper::Method::GET)
                        .and_path("/author/{authorname:[^/]*}")
                        .and_handler(author::http_handler),
                )
                .route(
                    route::first::Route::with_method(&hyper::Method::POST)
                        .and_path("/login")
                        .and_handler(login::http_handler),
                )
                .route(
                    route::first::Route::with_method(&hyper::Method::GET)
                        .and_path("/me")
                        .and_handler(me::http_handler),
                )
                .route(
                    route::first::Route::with_any_methods()
                        .and_path("/{_:.*}")
                        .and_handler(api_not_found_fallback_handler),
                )
            },
        )
    })
}
