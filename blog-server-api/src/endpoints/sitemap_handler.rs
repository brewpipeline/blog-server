use chrono::{DateTime, FixedOffset, NaiveDateTime};
use sitemap_rs::url::{ChangeFrequency, Url};
use sitemap_rs::url_set::UrlSet;
use std::sync::Arc;

use crate::extensions::Resolve;
use blog_generic::entities::*;
use blog_server_services::traits::post_service::*;

use screw_core::request::*;
use screw_core::response::*;
use screw_core::routing::*;

const RECORDS_LIMIT: usize = 50000;

// TODO: split sitemaps / refactor
pub async fn sitemap_handler<Extensions: Resolve<Arc<Box<dyn PostService>>>>(
    request: router::RoutedRequest<Request<Extensions>>,
) -> Response {
    let post_service: Arc<Box<dyn PostService>> = request.origin.extensions.resolve();

    let posts = post_service
        .posts(PostsQuery {
            search_query: None,
            author_id: None,
            tag_id: None,
            publish_type: Some(&PublishType::Published),
            offset: &0,
            limit: &(RECORDS_LIMIT as u64),
        })
        .await
        .map(|p| p.posts)
        .unwrap_or_else(|_| vec![]);

    let mut urls = posts
        .into_iter()
        .map(|post| {
            Url::builder(format!(
                "{site_url}/post/{slug}/{id}",
                site_url = crate::SITE_URL,
                slug = post.base.slug,
                id = post.id,
            ))
            .last_modified(DateTime::from_naive_utc_and_offset(
                NaiveDateTime::from_timestamp_opt(post.base.created_at as i64 / 1000, 0).unwrap(),
                FixedOffset::east_opt(0).unwrap(),
            ))
            .change_frequency(ChangeFrequency::Daily)
            .priority(1.0)
            .build()
            .unwrap()
        })
        .collect::<Vec<Url>>();
    urls.truncate(RECORDS_LIMIT);

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
