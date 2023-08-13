use serde::Serialize;
use std::sync::Arc;

use crate::extensions::Resolve;
use blog_generic::entities;
use blog_server_services::traits::author_service::*;
use blog_server_services::traits::entity_post_service::*;
use blog_server_services::traits::post_service::*;

use screw_core::request::*;
use screw_core::response::*;
use screw_core::routing::*;

use blog_ui::*;

const INDEX_HTML: &str = include_str!("../../../index.html");

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
    Extensions: Resolve<Arc<Box<dyn AuthorService>>>
        + Resolve<Arc<Box<dyn PostService>>>
        + Resolve<Arc<Box<dyn EntityPostService>>>,
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
    update_tag(
        &mut html,
        TITLE_TAG_PREFIX,
        TITLE_TAG_SUFFIX,
        TITLE_TAG_BODY_PREFIX,
        TITLE_TAG_BODY_SUFFIX,
    );
    update_tag(
        &mut html,
        DESCRIPTION_TAG_PREFIX,
        DESCRIPTION_TAG_SUFFIX,
        DESCRIPTION_TAG_BODY_PREFIX,
        DESCRIPTION_TAG_BODY_SUFFIX,
    );
    update_tag(
        &mut html,
        KEYWORDS_TAG_PREFIX,
        KEYWORDS_TAG_SUFFIX,
        KEYWORDS_TAG_BODY_PREFIX,
        KEYWORDS_TAG_BODY_SUFFIX,
    );
    html
}

fn update_tag(
    html: &mut String,
    main_tag_prefix: &str,
    main_tag_suffix: &str,
    body_tag_prefix: &str,
    body_tag_suffix: &str,
) {
    let Some(content) = last_content(&html, body_tag_prefix, body_tag_suffix) else {
        return
    };
    let empty_tag = main_tag_prefix.to_string() + main_tag_suffix;
    let tag = {
        let mut tag = String::new();
        tag.push_str(main_tag_prefix);
        tag.push_str(content.as_str());
        tag.push_str(main_tag_suffix);
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

fn app_content_encode<E: Serialize>(entity: &E) -> Option<AppContent> {
    Some(AppContent {
        r#type: "application/json".to_string(),
        value: serde_json::to_string(entity).ok()?,
    })
}

async fn app_content<Extensions>(
    request: &router::RoutedRequest<Request<Extensions>>,
) -> Option<AppContent>
where
    Extensions: Resolve<Arc<Box<dyn AuthorService>>>
        + Resolve<Arc<Box<dyn PostService>>>
        + Resolve<Arc<Box<dyn EntityPostService>>>,
{
    match Route::recognize_path(request.path.as_str())? {
        Route::Post { slug: _, id } | Route::EditPost { id } => {
            let post_service: Arc<Box<dyn PostService>> = request.origin.extensions.resolve();
            let entity_post_service: Arc<Box<dyn EntityPostService>> =
                request.origin.extensions.resolve();
            let post = post_service.post_by_id(&id).await.ok().flatten()?;
            let post_entity = entity_post_service
                .posts_entities(vec![post])
                .await
                .ok()?
                .remove(0);
            app_content_encode(&post_entity)
        }
        Route::Author { slug } => {
            let author_service: Arc<Box<dyn AuthorService>> = request.origin.extensions.resolve();
            let author_entity: entities::Author = author_service
                .author_by_slug(&slug)
                .await
                .ok()
                .flatten()?
                .into();
            app_content_encode(&author_entity)
        }
        _ => None,
    }
}
