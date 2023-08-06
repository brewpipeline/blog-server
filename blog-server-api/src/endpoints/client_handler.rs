use serde::Serialize;
use std::sync::Arc;

use crate::extensions::Resolve;
use blog_generic::entities;
use blog_server_services::traits::author_service::*;
use blog_server_services::traits::post_service::*;

use screw_components::dyn_result::*;
use screw_core::request::*;
use screw_core::response::*;
use screw_core::routing::*;

use blog_ui::*;

const INDEX_HTML: &str = include_str!("../../../../blog-ui/dist/index.html");

const APP_TAG_PREFIX: &str = "<div id=\"app\">";

const TITLE_TAG_PREFIX: &str = "<title>";
const TITLE_TAG_SUFFIX: &str = "</title>";
const DESCRIPTION_TAG_PREFIX: &str = "<meta name=\"description\" content=\"";
const DESCRIPTION_TAG_SUFFIX: &str = "\">";
const KEYWORDS_TAG_PREFIX: &str = "<meta name=\"keywords\" content=\"";
const KEYWORDS_TAG_SUFFIX: &str = "\">";

const TITLE_TAG_BODY_PREFIX: &str = "<script data-page-content=\"title\" type=\"text/plain\">";
const TITLE_TAG_BODY_SUFFIX: &str = "</script>";
const DESCRIPTION_TAG_BODY_PREFIX: &str =
    "<script data-page-content=\"description\" type=\"text/plain\">";
const DESCRIPTION_TAG_BODY_SUFFIX: &str = "</script>";
const KEYWORDS_TAG_BODY_PREFIX: &str =
    "<script data-page-content=\"keywords\" type=\"text/plain\">";
const KEYWORDS_TAG_BODY_SUFFIX: &str = "</script>";

pub async fn client_handler<
    Extensions: Resolve<Arc<Box<dyn AuthorService>>> + Resolve<Arc<Box<dyn PostService>>>,
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
            .status(hyper::StatusCode::OK)
            .header("Content-Type", "text/html")
            .body(hyper::Body::from(page))
            .unwrap(),
    }
}

fn update_meta(mut html: String) -> String {
    html = update_tag(
        html,
        TITLE_TAG_PREFIX,
        TITLE_TAG_SUFFIX,
        TITLE_TAG_BODY_PREFIX,
        TITLE_TAG_BODY_SUFFIX,
    );
    html = update_tag(
        html,
        DESCRIPTION_TAG_PREFIX,
        DESCRIPTION_TAG_SUFFIX,
        DESCRIPTION_TAG_BODY_PREFIX,
        DESCRIPTION_TAG_BODY_SUFFIX,
    );
    html = update_tag(
        html,
        KEYWORDS_TAG_PREFIX,
        KEYWORDS_TAG_SUFFIX,
        KEYWORDS_TAG_BODY_PREFIX,
        KEYWORDS_TAG_BODY_SUFFIX,
    );
    html
}

fn update_tag(
    html: String,
    main_tag_prefix: &str,
    main_tag_suffix: &str,
    body_tag_prefix: &str,
    body_tag_suffix: &str,
) -> String {
    let Some(content) = content(&html, body_tag_prefix, body_tag_suffix) else {
        return html
    };
    let empty_tag = main_tag_prefix.to_string() + main_tag_suffix;
    let tag = {
        let mut tag = String::new();
        tag.push_str(main_tag_prefix);
        tag.push_str(content.as_str());
        tag.push_str(main_tag_suffix);
        tag
    };
    html.replace(empty_tag.as_str(), tag.as_str())
}

fn content(html: &String, prefix: &str, suffix: &str) -> Option<String> {
    let parts = html.split(prefix);
    let mut content = parts.last()?.to_owned();
    content = content.split_once(suffix)?.0.to_owned();
    Some(content)
}

async fn app_content<
    Extensions: Resolve<Arc<Box<dyn AuthorService>>> + Resolve<Arc<Box<dyn PostService>>>,
>(
    request: &router::RoutedRequest<Request<Extensions>>,
) -> Option<AppContent> {
    let Some(route) = Route::recognize_path(request.path.as_str()) else {
        return None
    };

    fn json_encode<D: Into<E>, E: Serialize>(data: DResult<Option<D>>) -> Option<String> {
        let entity: Option<E> = data.ok().flatten().map(|d| d.into());
        entity.map(|e| serde_json::to_string(&e).ok()).flatten()
    }

    let json = match route {
        Route::Post { slug: _, id } | Route::EditPost { id } => {
            let post_service: Arc<Box<dyn PostService>> = request.origin.extensions.resolve();
            let post = post_service.post_by_id(&id).await;
            json_encode::<_, entities::Post>(post)
        }
        Route::Author { slug } => {
            let auth_service: Arc<Box<dyn AuthorService>> = request.origin.extensions.resolve();
            let author = auth_service.author_by_slug(&slug).await;
            json_encode::<_, entities::Author>(author)
        }
        _ => None,
    };

    json.map(|json| AppContent {
        r#type: "application/json".to_string(),
        value: json,
    })
}
