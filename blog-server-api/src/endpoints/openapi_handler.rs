use screw_core::request::*;
use screw_core::response::*;
use screw_core::routing::*;

pub const OPENAPI_PATH: &str = "/openapi.json";
pub const OPENAPI_MEDIA_TYPE: &str = "application/vnd.oai.openapi+json;version=3.1";

static OPENAPI: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    let mut document: serde_json::Value = serde_json::from_str(include_str!("openapi.json"))
        .unwrap_or_else(|e| panic!("failed to parse openapi.json: {e}"));

    document["servers"] = serde_json::json!([{
        "url": format!("{site_url}/api", site_url = &*crate::SITE_URL),
    }]);

    if let Some(paths) = document["paths"].as_object_mut() {
        for (path, enabled) in [
            ("/chatgpt", cfg!(feature = "chatgpt")),
            ("/ylogin", cfg!(feature = "yandex")),
            ("/tlogin", cfg!(feature = "telegram")),
        ] {
            if !enabled {
                paths.remove(path);
            }
        }
    }

    serde_json::to_string(&document).expect("failed to serialize openapi document")
});

pub async fn openapi_handler<Extensions>(
    _: router::RoutedRequest<Request<Extensions>>,
) -> Response {
    Response {
        http: hyper::Response::builder()
            .header("Content-Type", OPENAPI_MEDIA_TYPE)
            .header("Cache-Control", "public, max-age=3600")
            .body(screw_core::body::full(OPENAPI.as_str()))
            .unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn document() -> serde_json::Value {
        unsafe { std::env::set_var("SITE_URL", "https://example.com") };
        serde_json::from_str(&OPENAPI).unwrap()
    }

    #[test]
    fn servers_point_at_the_running_site() {
        assert_eq!(document()["servers"][0]["url"], "https://example.com/api");
    }

    #[test]
    fn feature_gated_paths_follow_the_build() {
        let document = document();
        let paths = document["paths"].as_object().unwrap();
        assert_eq!(paths.contains_key("/chatgpt"), cfg!(feature = "chatgpt"));
        assert_eq!(paths.contains_key("/ylogin"), cfg!(feature = "yandex"));
        assert_eq!(paths.contains_key("/tlogin"), cfg!(feature = "telegram"));
    }

    #[test]
    fn always_available_paths_are_described() {
        let document = document();
        let paths = document["paths"].as_object().unwrap();
        for path in [
            "/posts",
            "/post",
            "/post/{id}",
            "/authors",
            "/author/slug/{slug}",
            "/author/me",
            "/comments/{post_id}",
            "/comment",
            "/login",
        ] {
            assert!(paths.contains_key(path), "{path} is missing");
        }
    }

    #[test]
    fn every_reference_resolves() {
        let document = document();
        let mut references = Vec::new();
        collect_references(&document, &mut references);
        assert!(!references.is_empty());

        for reference in references {
            let mut node = &document;
            for segment in reference.trim_start_matches("#/").split('/') {
                node = node
                    .get(segment)
                    .unwrap_or_else(|| panic!("{reference} does not resolve"));
            }
        }
    }

    fn collect_references(node: &serde_json::Value, out: &mut Vec<String>) {
        match node {
            serde_json::Value::Object(map) => {
                for (key, value) in map {
                    match value.as_str() {
                        Some(reference) if key == "$ref" => out.push(reference.to_string()),
                        _ => collect_references(value, out),
                    }
                }
            }
            serde_json::Value::Array(items) => {
                for item in items {
                    collect_references(item, out);
                }
            }
            _ => {}
        }
    }
}
