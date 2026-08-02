use crate::endpoints::*;
use crate::extensions::Resolve;
use crate::utils::accept::prefers_markdown;
use blog_server_services::traits::author_service::*;
use blog_server_services::traits::entity_post_service::*;
use blog_server_services::traits::post_service::*;

use screw_core::request::*;
use screw_core::response::*;
use screw_core::routing::*;

use blog_generic::*;
use blog_ui::*;

use super::api_catalog_handler::{API_CATALOG_MEDIA_TYPE, API_CATALOG_PATH};
use super::markdown_handler::{MARKDOWN_CONTENT_TYPE, markdown_page};
use super::openapi_handler::{OPENAPI_MEDIA_TYPE, OPENAPI_PATH};

static DISCOVERY_LINK_HEADER: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    format!(
        "<{API_CATALOG_PATH}>; rel=\"api-catalog\"; type=\"{API_CATALOG_MEDIA_TYPE}\", \
         <{OPENAPI_PATH}>; rel=\"service-desc\"; type=\"{OPENAPI_MEDIA_TYPE}\""
    )
});

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
    if prefers_markdown(request.origin.http.headers()) {
        if let Some((status, markdown)) = markdown_page(&request).await {
            return Response {
                http: hyper::Response::builder()
                    .status(status)
                    .header("Content-Type", MARKDOWN_CONTENT_TYPE)
                    .header("Vary", "Accept")
                    .header("Link", DISCOVERY_LINK_HEADER.as_str())
                    .body(screw_core::body::full(markdown))
                    .unwrap(),
            };
        }
    }

    let (before, after) = INDEX_HTML.split_once(APP_TAG_PREFIX).unwrap();

    let (status, app_content) = resolve_page::<_, DefaultPageProcessor>(&request).await;

    let rendered = server_renderer(
        render_path(request.path.as_str()),
        request.query.as_map().clone(),
        app_content,
    )
    .render()
    .await;

    let page = update_meta(format!("{before}{APP_TAG_PREFIX}{rendered}{after}"));

    Response {
        http: hyper::Response::builder()
            .status(status)
            .header("Content-Type", "text/html")
            .header("Vary", "Accept")
            .header("Link", DISCOVERY_LINK_HEADER.as_str())
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

fn render_path(path: &str) -> String {
    if path.starts_with('/') && !path.contains('?') && !path.contains('#') {
        path.to_string()
    } else {
        "/404".to_string()
    }
}

async fn encoded<C, E: serde::Serialize>(
    container: impl std::future::Future<Output = Option<C>>,
    select: impl FnOnce(C) -> E,
) -> Option<AppContent> {
    Some(AppContent {
        r#type: "application/json".to_string(),
        value: serde_json::to_string(&select(container.await?)).ok()?,
    })
}

async fn resolve_page<Extensions, PP>(
    request: &router::RoutedRequest<Request<Extensions>>,
) -> (hyper::StatusCode, Option<AppContent>)
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

    match Route::recognize_path(request.path.as_str()) {
        Some(Route::Post { slug, id }) => {
            let content = encoded(
                async {
                    post::direct_handler(id, ext.resolve(), ext.resolve())
                        .await
                        .filter(|c| c.post.id == id && c.post.slug == slug)
                },
                |c| c.post,
            )
            .await;
            (page_status(content.is_some()), content)
        }
        Some(Route::Author { slug }) => {
            let content = encoded(author::direct_handler(slug, ext.resolve()), |c| c.author).await;
            (page_status(content.is_some()), content)
        }
        Some(Route::Tag { slug, id }) => {
            let content = encoded(
                async {
                    tag::direct_handler(id, ext.resolve())
                        .await
                        .filter(|c| c.tag.id == id && c.tag.slug == slug)
                },
                |c| c.tag,
            )
            .await;
            (page_status(content.is_some()), content)
        }
        Some(Route::Posts) => {
            let container = posts::direct_handler(
                page_processor.offset(),
                page_processor.limit(),
                ext.resolve(),
                ext.resolve(),
            )
            .await;
            let is_empty = container.as_ref().map_or(true, |c| c.posts.is_empty());
            let content = encoded(std::future::ready(container), |c| c).await;
            (page_status(!is_empty), content)
        }
        Some(Route::Authors) => {
            let container = authors::direct_handler(
                page_processor.offset(),
                page_processor.limit(),
                ext.resolve(),
            )
            .await;
            let is_empty = container.as_ref().map_or(true, |c| c.authors.is_empty());
            let content = encoded(std::future::ready(container), |c| c).await;
            (page_status(!is_empty), content)
        }
        None | Some(Route::NotFound) => (hyper::StatusCode::NOT_FOUND, None),
        Some(_) => (hyper::StatusCode::OK, None),
    }
}

fn page_status(found: bool) -> hyper::StatusCode {
    if found {
        hyper::StatusCode::OK
    } else {
        hyper::StatusCode::NOT_FOUND
    }
}
