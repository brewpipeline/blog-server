use super::endpoints::*;
use super::extensions::*;
use screw_api::json::*;
use screw_api::request::*;
use screw_api::response::*;
use screw_core::request::*;
use screw_core::response::*;
use screw_core::routing::*;

#[cfg(not(feature = "ssr"))]
async fn not_found_fallback_handler<Extensions>(
    _: router::RoutedRequest<Request<Extensions>>,
) -> Response {
    Response {
        http: hyper::Response::builder()
            .status(hyper::StatusCode::NOT_FOUND)
            .body(screw_core::body::empty())
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
    #[cfg(not(feature = "ssr"))]
    let fallback_handler = not_found_fallback_handler;
    #[cfg(feature = "ssr")]
    let fallback_handler = client_handler;

    #[cfg(not(feature = "ssr"))]
    let sitemap_handler = not_found_fallback_handler;
    #[cfg(feature = "ssr")]
    let sitemap_handler = sitemap_handler;

    #[cfg(not(feature = "yandex"))]
    let yandex_handler = api_not_found_fallback_handler;
    #[cfg(feature = "yandex")]
    let yandex_handler = yandex_login::http_handler;

    #[cfg(not(feature = "telegram"))]
    let telegram_handler = api_not_found_fallback_handler;
    #[cfg(feature = "telegram")]
    let telegram_handler = telegram_login::http_handler;

    #[cfg(not(feature = "chatgpt"))]
    let chatgpt_handler = api_not_found_fallback_handler;
    #[cfg(feature = "chatgpt")]
    let chatgpt_handler = chatgpt::http_handler;

    router::first::Router::with_fallback_handler(fallback_handler).and_routes(|r| {
        r.scoped_middleware(
            "/api",
            JsonApiMiddlewareConverter {
                pretty_printed: cfg!(debug_assertions),
            },
            |r| {
                r.scoped("/author", |r| {
                    r.route(
                        route::first::Route::with_method(&hyper::Method::GET)
                            .and_path("/me")
                            .and_handler(author_me::http_handler),
                    )
                    .route(
                        route::first::Route::with_method(&hyper::Method::GET)
                            .and_path("/slug/{slug:[^/]*}")
                            .and_handler(author::http_handler),
                    )
                    .route(
                        route::first::Route::with_method(&hyper::Method::GET)
                            .and_path("/id/{id:[^/]*}/block")
                            .and_handler(author_block::http_handler_block),
                    )
                    .route(
                        route::first::Route::with_method(&hyper::Method::GET)
                            .and_path("/id/{id:[^/]*}/unblock")
                            .and_handler(author_block::http_handler_unblock),
                    )
                    .route(
                        route::first::Route::with_method(&hyper::Method::PATCH)
                            .and_path("/id/{id:[^/]*}/subscribe")
                            .and_handler(author_subscribe::http_handler_subscribe),
                    )
                    .route(
                        route::first::Route::with_method(&hyper::Method::PATCH)
                            .and_path("/id/{id:[^/]*}/unsubscribe")
                            .and_handler(author_subscribe::http_handler_unsubscribe),
                    )
                    .route(
                        route::first::Route::with_method(&hyper::Method::PATCH)
                            .and_path("/reset_override_social_data")
                            .and_handler(author_override_social_data::http_handler_disabled),
                    )
                    .route(
                        route::first::Route::with_method(&hyper::Method::PATCH)
                            .and_path("/minimal")
                            .and_handler(update_minimal_author::http_handler),
                    )
                    .route(
                        route::first::Route::with_method(&hyper::Method::PATCH)
                            .and_path("/secondary")
                            .and_handler(update_secondary_author::http_handler),
                    )
                })
                .scoped("/authors", |r| {
                    r.route(
                        route::first::Route::with_method(&hyper::Method::GET)
                            .and_path("/search/{query:[^/]*}")
                            .and_handler(authors::http_handler),
                    )
                    .route(
                        route::first::Route::with_method(&hyper::Method::GET)
                            .and_path("")
                            .and_handler(authors::http_handler),
                    )
                })
                .scoped("/post", |r| {
                    r.route(
                        route::first::Route::with_method(&hyper::Method::GET)
                            .and_path("/{id:[^/]*}")
                            .and_handler(post::http_handler),
                    )
                    .route(
                        route::first::Route::with_method(&hyper::Method::PATCH)
                            .and_path("/{id:[^/]*}")
                            .and_handler(update_post::http_handler),
                    )
                    .route(
                        route::first::Route::with_method(&hyper::Method::DELETE)
                            .and_path("/{id:[^/]*}")
                            .and_handler(delete_post::http_handler),
                    )
                    .route(
                        route::first::Route::with_method(&hyper::Method::POST)
                            .and_path("")
                            .and_handler(create_post::http_handler),
                    )
                    .route(
                        route::first::Route::with_method(&hyper::Method::GET)
                            .and_path("/{id:[^/]*}/recommendation")
                            .and_handler(post_recommendation::http_handler),
                    )
                    .route(
                        route::first::Route::with_method(&hyper::Method::PATCH)
                            .and_path("/{id:[^/]*}/recommended/true")
                            .and_handler(post_update_recommended::http_handler_true),
                    )
                    .route(
                        route::first::Route::with_method(&hyper::Method::PATCH)
                            .and_path("/{id:[^/]*}/recommended/false")
                            .and_handler(post_update_recommended::http_handler_false),
                    )
                })
                .scoped("/posts", |r| {
                    r.scoped("/unpublished", |r| {
                        r.route(
                            route::first::Route::with_method(&hyper::Method::GET)
                                .and_path("")
                                .and_handler(posts::http_handler_unpublished),
                        )
                    })
                    .scoped("/hidden", |r| {
                        r.route(
                            route::first::Route::with_method(&hyper::Method::GET)
                                .and_path("")
                                .and_handler(posts::http_handler_hidden),
                        )
                    })
                    .route(
                        route::first::Route::with_method(&hyper::Method::GET)
                            .and_path("")
                            .and_handler(posts::http_handler),
                    )
                })
                .route(
                    route::first::Route::with_method(&hyper::Method::GET)
                        .and_path("/tag/{id:[^/]*}")
                        .and_handler(tag::http_handler),
                )
                .route(
                    route::first::Route::with_method(&hyper::Method::GET)
                        .and_path("/comments/{post_id:[^/]*}")
                        .and_handler(comments::http_handler),
                )
                .scoped("/comment", |r| {
                    r.route(
                        route::first::Route::with_method(&hyper::Method::DELETE)
                            .and_path("/{id:[^/]*}")
                            .and_handler(delete_comment::http_handler),
                    )
                    .route(
                        route::first::Route::with_method(&hyper::Method::POST)
                            .and_path("")
                            .and_handler(create_comment::http_handler),
                    )
                })
                .route(
                    route::first::Route::with_method(&hyper::Method::POST)
                        .and_path("/chatgpt")
                        .and_handler(chatgpt_handler),
                )
                .route(
                    route::first::Route::with_method(&hyper::Method::POST)
                        .and_path("/login")
                        .and_handler(login::http_handler),
                )
                .route(
                    route::first::Route::with_method(&hyper::Method::POST)
                        .and_path("/ylogin")
                        .and_handler(yandex_handler),
                )
                .route(
                    route::first::Route::with_method(&hyper::Method::POST)
                        .and_path("/tlogin")
                        .and_handler(telegram_handler),
                )
                .route(
                    route::first::Route::with_any_method()
                        .and_path("/{_:.*}")
                        .and_handler(api_not_found_fallback_handler),
                )
            },
        )
        .route(
            route::first::Route::with_method(&hyper::Method::GET)
                .and_path("/sitemap.xml")
                .and_handler(sitemap_handler),
        )
    })
}
