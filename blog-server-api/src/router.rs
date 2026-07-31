use super::endpoints::*;
use super::extensions::*;
use super::utils::auth_middleware::{AuthApiMiddleware, AuthPolicy, OptionalAuthApiMiddleware};
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
    fn status_code(&self) -> hyper::StatusCode {
        hyper::StatusCode::NOT_FOUND
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

pub fn make_router<Extensions: ExtensionsProviderType>()
-> router::second::Router<Request<Extensions>, Response> {
    #[cfg(not(feature = "ssr"))]
    let fallback_handler = not_found_fallback_handler;
    #[cfg(feature = "ssr")]
    let fallback_handler = client_handler;

    #[cfg(not(feature = "ssr"))]
    let sitemap_handler = not_found_fallback_handler;
    #[cfg(feature = "ssr")]
    let sitemap_handler = sitemap_handler;

    #[cfg(not(feature = "ssr"))]
    let robots_handler = not_found_fallback_handler;
    #[cfg(feature = "ssr")]
    let robots_handler = robots_handler;

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
                ..Default::default()
            },
            |r| {
                r.scoped("/author", |r| {
                    r.route(
                        route::first::Route::with_method(&hyper::Method::GET)
                            .and_path("/slug/{slug:[^/]*}")
                            .and_handler(author::http_handler),
                    )
                    .middleware(
                        AuthApiMiddleware::with_policy(AuthPolicy::Authenticated),
                        |r| {
                            r.route(
                                route::first::Route::with_method(&hyper::Method::GET)
                                    .and_path("/me")
                                    .and_handler(author_me::http_handler),
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
                        },
                    )
                    .middleware(AuthApiMiddleware::with_policy(AuthPolicy::Editor), |r| {
                        r.route(
                            route::first::Route::with_method(&hyper::Method::GET)
                                .and_path("/id/{id:[^/]*}/block")
                                .and_handler(author_block::http_handler_block),
                        )
                        .route(
                            route::first::Route::with_method(&hyper::Method::GET)
                                .and_path("/id/{id:[^/]*}/unblock")
                                .and_handler(author_block::http_handler_unblock),
                        )
                    })
                    .middleware(
                        AuthApiMiddleware::with_policy(AuthPolicy::NotBlocked),
                        |r| {
                            r.route(
                                route::first::Route::with_method(&hyper::Method::PATCH)
                                    .and_path("/reset_override_social_data")
                                    .and_handler(
                                        author_override_social_data::http_handler_disabled,
                                    ),
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
                        },
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
                    r.middleware(OptionalAuthApiMiddleware, |r| {
                        r.route(
                            route::first::Route::with_method(&hyper::Method::GET)
                                .and_path("/{id:[^/]*}")
                                .and_handler(post::http_handler),
                        )
                    })
                    .route(
                        route::first::Route::with_method(&hyper::Method::GET)
                            .and_path("/{id:[^/]*}/recommendation")
                            .and_handler(post_recommendation::http_handler),
                    )
                    .middleware(
                        AuthApiMiddleware::with_policy(AuthPolicy::NotBlocked),
                        |r| {
                            r.route(
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
                        },
                    )
                    .middleware(
                        AuthApiMiddleware::with_policy(AuthPolicy::Editor),
                        |r| {
                            r.route(
                                route::first::Route::with_method(&hyper::Method::PATCH)
                                    .and_path("/{id:[^/]*}/recommended/true")
                                    .and_handler(post_update_recommended::http_handler_true),
                            )
                            .route(
                                route::first::Route::with_method(&hyper::Method::PATCH)
                                    .and_path("/{id:[^/]*}/recommended/false")
                                    .and_handler(post_update_recommended::http_handler_false),
                            )
                        },
                    )
                })
                .scoped("/posts", |r| {
                    r.middleware(
                        AuthApiMiddleware::with_policy(AuthPolicy::Authenticated),
                        |r| {
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
                        },
                    )
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
                    r.middleware(
                        AuthApiMiddleware::with_policy(AuthPolicy::NotBlocked),
                        |r| {
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
                        },
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
        .route(
            route::first::Route::with_method(&hyper::Method::GET)
                .and_path("/robots.txt")
                .and_handler(robots_handler),
        )
        .route(
            route::first::Route::with_method(&hyper::Method::GET)
                .and_path(api_catalog_handler::API_CATALOG_PATH)
                .and_handler(api_catalog_handler),
        )
        .route(
            route::first::Route::with_method(&hyper::Method::GET)
                .and_path(openapi_handler::OPENAPI_PATH)
                .and_handler(openapi_handler),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use blog_generic::events::{NewPostPublished, SubscriptionStateChanged};
    use blog_server_services::traits::Publish;
    use blog_server_services::traits::author_service::AuthorService;
    use blog_server_services::traits::comment_service::CommentService;
    use blog_server_services::traits::entity_comment_service::EntityCommentService;
    use blog_server_services::traits::entity_post_service::EntityPostService;
    use blog_server_services::traits::post_service::PostService;
    use blog_server_services::traits::social_service::SocialService;
    use std::sync::Arc;

    struct StubExtensions;

    macro_rules! stub_resolve {
        ($service:ty) => {
            impl Resolve<Arc<$service>> for StubExtensions {
                fn resolve(&self) -> Arc<$service> {
                    unimplemented!()
                }
            }
        };
    }

    stub_resolve!(dyn AuthorService);
    stub_resolve!(dyn PostService);
    stub_resolve!(dyn CommentService);
    stub_resolve!(dyn EntityCommentService);
    stub_resolve!(dyn EntityPostService);
    stub_resolve!(dyn SocialService);
    stub_resolve!(dyn Publish<NewPostPublished>);
    stub_resolve!(dyn Publish<SubscriptionStateChanged>);

    impl ExtensionsProviderType for StubExtensions {}

    #[test]
    fn router_builds_without_route_conflicts() {
        let _ = make_router::<StubExtensions>();
    }

    async fn serve() -> std::net::SocketAddr {
        unsafe { std::env::set_var("SITE_URL", "https://example.com") };

        let server_service = Arc::new(screw_core::server::ServerService::with_responder_factory(
            screw_core::responder_factory::ResponderFactory::with_router(make_router())
                .and_extensions(StubExtensions),
        ));

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            loop {
                let (stream, remote_addr) = listener.accept().await.unwrap();
                let session_service = server_service.make_session_service(remote_addr);
                tokio::spawn(async move {
                    let _ = hyper::server::conn::http1::Builder::new()
                        .serve_connection(hyper_util::rt::TokioIo::new(stream), session_service)
                        .await;
                });
            }
        });

        addr
    }

    async fn get(addr: std::net::SocketAddr, path: &str) -> String {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};

        let mut stream = tokio::net::TcpStream::connect(addr).await.unwrap();
        stream
            .write_all(
                format!("GET {path} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n")
                    .as_bytes(),
            )
            .await
            .unwrap();

        let mut response = Vec::new();
        stream.read_to_end(&mut response).await.unwrap();
        String::from_utf8_lossy(&response).into_owned()
    }

    #[tokio::test]
    async fn api_catalog_is_served_from_well_known() {
        let addr = serve().await;
        let response = get(addr, api_catalog_handler::API_CATALOG_PATH).await;

        assert!(response.starts_with("HTTP/1.1 200 OK"), "{response}");
        assert!(response.contains("content-type: application/linkset+json"));
        assert!(response.contains("\"anchor\":\"https://example.com/api\""));
    }

    #[tokio::test]
    async fn openapi_document_is_served() {
        let addr = serve().await;
        let response = get(addr, openapi_handler::OPENAPI_PATH).await;

        assert!(response.starts_with("HTTP/1.1 200 OK"), "{response}");
        assert!(response.contains("content-type: application/vnd.oai.openapi+json;version=3.1"));
        assert!(response.contains("\"openapi\":\"3.1.0\""));
    }
}
