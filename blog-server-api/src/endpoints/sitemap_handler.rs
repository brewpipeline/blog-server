use chrono::{DateTime, FixedOffset, NaiveDateTime};
use sitemap_rs::url::{ChangeFrequency, Url};
use sitemap_rs::url_set::UrlSet;
use std::sync::Arc;

use crate::extensions::Resolve;
use blog_server_services::traits::post_service::*;

use screw_core::request::*;
use screw_core::response::*;
use screw_core::routing::*;

fn error_response(error_text: &'static str) -> Response {
    Response {
        http: hyper::Response::builder()
            .status(hyper::StatusCode::INTERNAL_SERVER_ERROR)
            .header("Content-Type", "text/plain")
            .body(hyper::Body::from(error_text))
            .unwrap(),
    }
}

// TODO: split sitemaps / refactor
pub async fn sitemap_handler<Extensions: Resolve<Arc<Box<dyn PostService>>>>(
    request: router::RoutedRequest<Request<Extensions>>,
) -> Response {
    let post_service: Arc<Box<dyn PostService>> = request.origin.extensions.resolve();

    let Ok(posts) = post_service.posts(&0, &50000).await else {
        return error_response("database error");
    };

    let urls = posts
        .into_iter()
        .map(|p| {
            Url::builder(format!(
                "{site_url}/post/{slug}/{id}",
                site_url = crate::SITE_URL,
                slug = p.base.slug,
                id = p.id,
            ))
            .last_modified(DateTime::from_naive_utc_and_offset(
                NaiveDateTime::from_timestamp_opt(p.base.created_at as i64 / 1000, 0).unwrap(),
                FixedOffset::east_opt(0).unwrap(),
            ))
            .change_frequency(ChangeFrequency::Daily)
            .priority(1.0)
            .build()
            .unwrap()
        })
        .collect::<Vec<Url>>();

    let url_set: UrlSet = UrlSet::new(urls).unwrap();
    let mut buf = Vec::<u8>::new();
    url_set.write(&mut buf).unwrap();

    Response {
        http: hyper::Response::builder()
            .header("Content-Type", "application/xml")
            .body(hyper::Body::from(buf))
            .unwrap(),
    }
}
