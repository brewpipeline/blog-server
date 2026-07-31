use screw_core::request::*;
use screw_core::response::*;
use screw_core::routing::*;

use super::openapi_handler::{OPENAPI_MEDIA_TYPE, OPENAPI_PATH};

pub const API_CATALOG_PATH: &str = "/.well-known/api-catalog";
pub const API_CATALOG_MEDIA_TYPE: &str = "application/linkset+json";

static API_CATALOG: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    let site_url = &*crate::SITE_URL;
    let linkset = serde_json::json!({
        "linkset": [
            {
                "anchor": format!("{site_url}/api"),
                "service-desc": [
                    {
                        "href": format!("{site_url}{OPENAPI_PATH}"),
                        "type": OPENAPI_MEDIA_TYPE,
                    },
                ],
                "describedby": [
                    {
                        "href": format!("{site_url}{OPENAPI_PATH}"),
                        "type": OPENAPI_MEDIA_TYPE,
                    },
                ],
            },
        ],
    });
    serde_json::to_string(&linkset).expect("failed to serialize api catalog")
});

pub async fn api_catalog_handler<Extensions>(
    _: router::RoutedRequest<Request<Extensions>>,
) -> Response {
    Response {
        http: hyper::Response::builder()
            .header("Content-Type", API_CATALOG_MEDIA_TYPE)
            .header("Cache-Control", "public, max-age=3600")
            .body(screw_core::body::full(API_CATALOG.as_str()))
            .unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_anchors_the_api_and_points_at_the_description() {
        unsafe { std::env::set_var("SITE_URL", "https://example.com") };
        let catalog: serde_json::Value = serde_json::from_str(&API_CATALOG).unwrap();
        let entry = &catalog["linkset"][0];

        assert_eq!(entry["anchor"], "https://example.com/api");
        assert_eq!(
            entry["service-desc"][0]["href"],
            "https://example.com/openapi.json"
        );
        assert_eq!(entry["service-desc"][0]["type"], OPENAPI_MEDIA_TYPE);
    }
}
