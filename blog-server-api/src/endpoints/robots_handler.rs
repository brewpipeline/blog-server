use screw_core::request::*;
use screw_core::response::*;
use screw_core::routing::*;

// Served via nginx exclusion -> proxied to the server so the absolute
// `Sitemap:` URL can be built from the runtime SITE_URL.
pub async fn robots_handler<Extensions>(_: router::RoutedRequest<Request<Extensions>>) -> Response {
    let body = format!(
        "User-agent: *\n\
         Allow: /\n\
         Disallow: /settings\n\
         Disallow: /post/new\n\
         Disallow: /post/edit/\n\
         Disallow: /posts/unpublished\n\
         Disallow: /posts/my/unpublished\n\
         Disallow: /posts/hidden\n\
         Disallow: /posts/search\n\
         Disallow: /authors/search\n\
         \n\
         Sitemap: {site_url}/sitemap.xml\n",
        site_url = &*crate::SITE_URL,
    );

    Response {
        http: hyper::Response::builder()
            .header("Content-Type", "text/plain; charset=utf-8")
            .body(screw_core::body::full(body))
            .unwrap(),
    }
}
