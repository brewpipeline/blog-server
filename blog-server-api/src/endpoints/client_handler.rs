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

const INDEX_HTML: &str = include_str!("../../../index.html");

const APP_TAG_PREFIX: &str = "<div id=\"app\">";

const TITLE_TAG: [&str; 2] = ["<title>", "</title>"];
const DESCRIPTION_TAG: [&str; 2] = ["<meta name=\"description\" content=\"", "\">"];
const KEYWORDS_TAG: [&str; 2] = ["<meta name=\"keywords\" content=\"", "\">"];
const ROBOTS_TAG: [&str; 2] = ["<meta name=\"robots\" content=\"", "\">"];
const OG_TITLE_TAG: [&str; 2] = ["<meta property=\"og:title\" content=\"", "\">"];
const OG_DESCRIPTION_TAG: [&str; 2] = ["<meta property=\"og:description\" content=\"", "\">"];
const OG_TYPE_TAG: [&str; 2] = ["<meta property=\"og:type\" content=\"", "\">"];
const OG_IMAGE_TAG: [&str; 2] = ["<meta property=\"og:image\" content=\"", "\">"];
const OG_IMAGE_WIDTH_TAG: [&str; 2] = ["<meta property=\"og:image:width\" content=\"", "\">"];
const OG_IMAGE_HEIGHT_TAG: [&str; 2] = ["<meta property=\"og:image:height\" content=\"", "\">"];
const OG_SITE_NAME_TAG: [&str; 2] = ["<meta property=\"og:site_name\" content=\"", "\">"];

const TYPE_TAG_BODY: [&str; 2] = [
    "<script data-page-content=\"type\" type=\"text/plain\">",
    "</script>",
];
const TITLE_TAG_BODY: [&str; 2] = [
    "<script data-page-content=\"title\" type=\"text/plain\">",
    "</script>",
];
const SHORT_TITLE_TAG_BODY: [&str; 2] = [
    "<script data-page-content=\"short_title\" type=\"text/plain\">",
    "</script>",
];
const DESCRIPTION_TAG_BODY: [&str; 2] = [
    "<script data-page-content=\"description\" type=\"text/plain\">",
    "</script>",
];
const KEYWORDS_TAG_BODY: [&str; 2] = [
    "<script data-page-content=\"keywords\" type=\"text/plain\">",
    "</script>",
];
const IMAGE_TAG_BODY: [&str; 2] = [
    "<script data-page-content=\"image\" type=\"text/plain\">",
    "</script>",
];
const IMAGE_WIDTH_TAG_BODY: [&str; 2] = [
    "<script data-page-content=\"image_width\" type=\"text/plain\">",
    "</script>",
];
const IMAGE_HEIGHT_TAG_BODY: [&str; 2] = [
    "<script data-page-content=\"image_height\" type=\"text/plain\">",
    "</script>",
];
const ROBOTS_TAG_BODY: [&str; 2] = [
    "<script data-page-content=\"robots\" type=\"text/plain\">",
    "</script>",
];
const SITE_NAME_TAG_BODY: [&str; 2] = [
    "<script data-page-content=\"site_name\" type=\"text/plain\">",
    "</script>",
];

pub async fn client_handler<
    Extensions: Resolve<std::sync::Arc<Box<dyn AuthorService>>>
        + Resolve<std::sync::Arc<Box<dyn PostService>>>
        + Resolve<std::sync::Arc<Box<dyn EntityPostService>>>,
>(
    request: router::RoutedRequest<Request<Extensions>>,
) -> Response {
    let (index_html_before, index_html_after) = {
        let (index_html_before, index_html_after) = INDEX_HTML.split_once(APP_TAG_PREFIX).unwrap();
        let mut index_html_before = index_html_before.to_owned();
        index_html_before.push_str(APP_TAG_PREFIX);
        let index_html_after = index_html_after.to_owned();
        (index_html_before, index_html_after)
    };

    let status = status(&request).await;
    let app_content = app_content(&request).await;

    let rendered = server_renderer(
        request.path.as_str().to_string(),
        request.query,
        app_content,
    )
    .render()
    .await;

    let page = {
        let mut page = String::new();
        page.push_str(index_html_before.as_str());
        page.push_str(rendered.as_str());
        page.push_str(index_html_after.as_str());
        update_meta(page)
    };

    Response {
        http: hyper::Response::builder()
            .status(status)
            .header("Content-Type", "text/html")
            .body(hyper::Body::from(page))
            .unwrap(),
    }
}

fn update_meta(mut html: String) -> String {
    update_tag(&mut html, TITLE_TAG, TITLE_TAG_BODY);
    update_tag(&mut html, DESCRIPTION_TAG, DESCRIPTION_TAG_BODY);
    update_tag(&mut html, KEYWORDS_TAG, KEYWORDS_TAG_BODY);
    update_tag(&mut html, ROBOTS_TAG, ROBOTS_TAG_BODY);
    update_tag(&mut html, OG_TITLE_TAG, SHORT_TITLE_TAG_BODY);
    update_tag(&mut html, OG_DESCRIPTION_TAG, DESCRIPTION_TAG_BODY);
    update_tag(&mut html, OG_TYPE_TAG, TYPE_TAG_BODY);
    update_tag(&mut html, OG_IMAGE_TAG, IMAGE_TAG_BODY);
    update_tag(&mut html, OG_IMAGE_WIDTH_TAG, IMAGE_WIDTH_TAG_BODY);
    update_tag(&mut html, OG_IMAGE_HEIGHT_TAG, IMAGE_HEIGHT_TAG_BODY);
    update_tag(&mut html, OG_SITE_NAME_TAG, SITE_NAME_TAG_BODY);
    html
}

fn update_tag(html: &mut String, main_tag: [&str; 2], body_tag: [&str; 2]) {
    let Some(content) = last_content(&html, body_tag[0], body_tag[1]) else {
        return;
    };
    let empty_tag = main_tag[0].to_string() + main_tag[1];
    let tag = {
        let mut tag = String::new();
        tag.push_str(main_tag[0]);
        tag.push_str(content.as_str());
        tag.push_str(main_tag[1]);
        tag
    };
    *html = html.replace(empty_tag.as_str(), tag.as_str());
}

fn last_content(html: &String, prefix: &str, suffix: &str) -> Option<String> {
    let parts = html.split(prefix);
    let mut content = parts.last()?.to_owned();
    content = content.split_once(suffix)?.0.to_owned();
    Some(content)
}

fn app_content_encode<E: serde::Serialize>(entity: &E) -> Option<AppContent> {
    Some(AppContent {
        r#type: "application/json".to_string(),
        value: serde_json::to_string(entity).ok()?,
    })
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
async fn app_content<Extensions>(
    request: &router::RoutedRequest<Request<Extensions>>,
) -> Option<AppContent>
where
    Extensions: Resolve<std::sync::Arc<Box<dyn AuthorService>>>
        + Resolve<std::sync::Arc<Box<dyn PostService>>>
        + Resolve<std::sync::Arc<Box<dyn EntityPostService>>>,
{
    let offset = || -> u64 {
        offset_for_page::<ITEMS_PER_PAGE>(
            &request
                .query
                .get("page")
                .map(|v| v.parse().ok())
                .flatten()
                .unwrap_or(1),
        )
    };
    let limit = || -> u64 { ITEMS_PER_PAGE };
    match Route::recognize_path(request.path.as_str())? {
        Route::Post { slug: _, id } | Route::EditPost { id } => post::direct_handler(
            id.to_string(),
            request.origin.extensions.resolve(),
            request.origin.extensions.resolve(),
        )
        .await
        .map(|v| app_content_encode(&v.post))
        .flatten(),
        Route::Author { slug } => author::direct_handler(slug, request.origin.extensions.resolve())
            .await
            .map(|v| app_content_encode(&v.author))
            .flatten(),
        Route::Tag { slug: _, id } => {
            tag::direct_handler(id.to_string(), request.origin.extensions.resolve())
                .await
                .map(|v| app_content_encode(&v.tag))
                .flatten()
        }
        Route::Posts => posts::direct_handler(
            offset(),
            limit(),
            request.origin.extensions.resolve(),
            request.origin.extensions.resolve(),
        )
        .await
        .map(|v| app_content_encode(&v))
        .flatten(),
        Route::Authors => {
            authors::direct_handler(offset(), limit(), request.origin.extensions.resolve())
                .await
                .map(|v| app_content_encode(&v))
                .flatten()
        }
        _ => None,
    }
}
