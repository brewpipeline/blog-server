use crate::endpoints::*;
use crate::extensions::Resolve;
use blog_server_services::traits::author_service::*;
use blog_server_services::traits::entity_post_service::*;
use blog_server_services::traits::post_service::*;

use screw_core::request::*;
use screw_core::response::*;
use screw_core::routing::*;

use blog_generic::*;
use blog_ui::*;

static INDEX_HTML: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    std::fs::read_to_string("dist/index.html")
        .unwrap_or_else(|e| panic!("failed to read dist/index.html: {e}"))
});

const APP_TAG_PREFIX: &str = "<div id=app>";

struct MetaRule {
    page_content_key: &'static str,
    head_tag: [&'static str; 2],
    wrap_as_content: bool,
}

const META_RULES: &[MetaRule] = &[
    MetaRule {
        page_content_key: "title",
        head_tag: ["<title>", "</title>"],
        wrap_as_content: false,
    },
    MetaRule {
        page_content_key: "description",
        head_tag: ["<meta name=description", ">"],
        wrap_as_content: true,
    },
    MetaRule {
        page_content_key: "keywords",
        head_tag: ["<meta name=keywords", ">"],
        wrap_as_content: true,
    },
    MetaRule {
        page_content_key: "robots",
        head_tag: ["<meta name=robots", ">"],
        wrap_as_content: true,
    },
    MetaRule {
        page_content_key: "short_title",
        head_tag: ["<meta property=og:title", ">"],
        wrap_as_content: true,
    },
    MetaRule {
        page_content_key: "description",
        head_tag: ["<meta property=og:description", ">"],
        wrap_as_content: true,
    },
    MetaRule {
        page_content_key: "type",
        head_tag: ["<meta property=og:type", ">"],
        wrap_as_content: true,
    },
    MetaRule {
        page_content_key: "image",
        head_tag: ["<meta property=og:image", ">"],
        wrap_as_content: true,
    },
    MetaRule {
        page_content_key: "image_width",
        head_tag: ["<meta property=og:image:width", ">"],
        wrap_as_content: true,
    },
    MetaRule {
        page_content_key: "image_height",
        head_tag: ["<meta property=og:image:height", ">"],
        wrap_as_content: true,
    },
    MetaRule {
        page_content_key: "site_name",
        head_tag: ["<meta property=og:site_name", ">"],
        wrap_as_content: true,
    },
];

pub async fn client_handler<
    Extensions: Resolve<std::sync::Arc<dyn AuthorService>>
        + Resolve<std::sync::Arc<dyn PostService>>
        + Resolve<std::sync::Arc<dyn EntityPostService>>,
>(
    request: router::RoutedRequest<Request<Extensions>>,
) -> Response {
    let (before, after) = INDEX_HTML.split_once(APP_TAG_PREFIX).unwrap();

    let status = status(&request).await;
    let app_content = app_content::<_, DefaultPageProcessor>(&request).await;

    let path = request.path.as_str();
    let render_path = if path.starts_with('/') && !path.contains('?') && !path.contains('#') {
        path.to_string()
    } else {
        "/404".to_string()
    };

    let rendered = server_renderer(render_path, request.query, app_content)
        .render()
        .await;

    let page = update_meta(format!("{before}{APP_TAG_PREFIX}{rendered}{after}"));

    Response {
        http: hyper::Response::builder()
            .status(status)
            .header("Content-Type", "text/html")
            .body(screw_core::body::full(page))
            .unwrap(),
    }
}

fn update_meta(mut html: String) -> String {
    for rule in META_RULES {
        update_tag(&mut html, rule);
    }
    html
}

fn update_tag(html: &mut String, rule: &MetaRule) {
    let body_prefix = format!(
        "<script data-page-content=\"{}\" type=\"text/plain\">",
        rule.page_content_key
    );
    let Some(content) = last_content(html, &body_prefix, "</script>") else {
        return;
    };
    let content = if rule.wrap_as_content {
        format!(" content=\"{content}\"")
    } else {
        content
    };

    let [open, close] = rule.head_tag;
    let empty_tag = format!("{open}{close}");
    let filled_tag = format!("{open}{content}{close}");
    *html = html.replace(&empty_tag, &filled_tag);
}

fn last_content(html: &str, prefix: &str, suffix: &str) -> Option<String> {
    let content = html.split(prefix).last()?;
    Some(content.split_once(suffix)?.0.to_owned())
}

fn app_content_encode<E: serde::Serialize>(entity: &E) -> Option<AppContent> {
    Some(AppContent {
        r#type: "application/json".to_string(),
        value: serde_json::to_string(entity).ok()?,
    })
}

async fn encoded<C, E: serde::Serialize>(
    container: impl std::future::Future<Output = Option<C>>,
    select: impl FnOnce(C) -> E,
) -> Option<AppContent> {
    app_content_encode(&select(container.await?))
}

async fn status<Extensions>(
    request: &router::RoutedRequest<Request<Extensions>>,
) -> hyper::StatusCode {
    if Route::recognize_path(request.path.as_str()).unwrap_or(Route::NotFound) != Route::NotFound {
        hyper::StatusCode::OK
    } else {
        hyper::StatusCode::NOT_FOUND
    }
}

// TODO: to think, if it's not a cringe
async fn app_content<Extensions, PP>(
    request: &router::RoutedRequest<Request<Extensions>>,
) -> Option<AppContent>
where
    Extensions: Resolve<std::sync::Arc<dyn AuthorService>>
        + Resolve<std::sync::Arc<dyn PostService>>
        + Resolve<std::sync::Arc<dyn EntityPostService>>,
    PP: PageProcessor,
{
    let page = request
        .query
        .get("page")
        .and_then(|v| v.parse().ok())
        .unwrap_or(1);
    let page_processor = PP::create_for_page(&page);
    let ext = &request.origin.extensions;

    match Route::recognize_path(request.path.as_str())? {
        Route::Post { slug: _, id } | Route::EditPost { id } => {
            encoded(
                post::direct_handler(id.to_string(), ext.resolve(), ext.resolve()),
                |c| c.post,
            )
            .await
        }
        Route::Author { slug } => {
            encoded(author::direct_handler(slug, ext.resolve()), |c| c.author).await
        }
        Route::Tag { slug: _, id } => {
            encoded(tag::direct_handler(id.to_string(), ext.resolve()), |c| {
                c.tag
            })
            .await
        }
        Route::Posts => {
            encoded(
                posts::direct_handler(
                    page_processor.offset(),
                    page_processor.limit(),
                    ext.resolve(),
                    ext.resolve(),
                ),
                |c| c,
            )
            .await
        }
        Route::Authors => {
            encoded(
                authors::direct_handler(
                    page_processor.offset(),
                    page_processor.limit(),
                    ext.resolve(),
                ),
                |c| c,
            )
            .await
        }
        _ => None,
    }
}
