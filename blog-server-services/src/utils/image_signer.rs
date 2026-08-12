use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::sync::{LazyLock, OnceLock};
use std::time::Duration;

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE;
use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::utils::map_in_pattern::MapInPattern;

type HmacSha256 = Hmac<Sha256>;

const MAX_CACHED_URLS: usize = 256;
const CACHED_TIMEOUT: Duration = Duration::from_secs(2);

static HTTP_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| {
    reqwest::Client::builder()
        .timeout(CACHED_TIMEOUT)
        .build()
        .unwrap_or_else(|_| reqwest::Client::new())
});

struct ImageSigner {
    processor_url: String,
    internal_processor_url: String,
    secret: String,
}

static IMAGE_SIGNER: OnceLock<ImageSigner> = OnceLock::new();

pub fn init(processor_url: String, internal_processor_url: String, secret: String) {
    let _ = IMAGE_SIGNER.set(ImageSigner {
        processor_url,
        internal_processor_url,
        secret,
    });
}

#[derive(Clone, Copy)]
pub enum ImageVariant {
    Normal,
    Small,
    Medium,
}

impl ImageVariant {
    fn path_part(&self) -> &'static str {
        match self {
            ImageVariant::Normal => "",
            ImageVariant::Small => "small/",
            ImageVariant::Medium => "medium/",
        }
    }
}

pub fn signed_image_url(image_url: &str, variant: ImageVariant) -> String {
    let Some(signer) = IMAGE_SIGNER.get() else {
        return image_url.to_string();
    };
    let base64_url = URL_SAFE.encode(image_url.as_bytes());
    let Ok(mut mac) = HmacSha256::new_from_slice(signer.secret.as_bytes()) else {
        return image_url.to_string();
    };
    mac.update(base64_url.as_bytes());
    let signature = hex::encode(mac.finalize().into_bytes());
    format!(
        "{processor_url}mirror/{path_part}{base64_url}?sig={signature}",
        processor_url = signer.processor_url,
        path_part = variant.path_part(),
    )
}

pub fn processed_image_urls(
    extra: &[(&str, ImageVariant)],
    content: Option<&str>,
) -> HashMap<String, String> {
    let urls = RefCell::new(HashMap::new());
    if IMAGE_SIGNER.get().is_none() {
        return urls.into_inner();
    }
    for (url, variant) in extra {
        if !url.is_empty() {
            urls.borrow_mut()
                .insert((*url).to_string(), signed_image_url(url, *variant));
        }
    }
    if let Some(content) = content {
        content.map_in_pattern(["<img", ">"], |tag| {
            tag.map_in_pattern(["src=\"", "\""], |url| {
                if !url.is_empty() {
                    urls.borrow_mut()
                        .insert(url.to_string(), signed_image_url(url, ImageVariant::Medium));
                }
                String::new()
            })
        });
    }
    urls.into_inner()
}

pub async fn cached_image_urls<'a>(
    processed_image_urls: impl IntoIterator<Item = &'a HashMap<String, String>>,
) -> HashMap<String, String> {
    let Some(signer) = IMAGE_SIGNER.get() else {
        return HashMap::new();
    };
    let mirror_urls: Vec<String> = processed_image_urls
        .into_iter()
        .flat_map(|map| map.values().cloned())
        .collect::<HashSet<String>>()
        .into_iter()
        .collect();
    if mirror_urls.is_empty() {
        return HashMap::new();
    }
    let mut cached = HashMap::new();
    for chunk in mirror_urls.chunks(MAX_CACHED_URLS) {
        let Ok(response) = HTTP_CLIENT
            .post(format!(
                "{internal_processor_url}mirror/cached",
                internal_processor_url = signer.internal_processor_url,
            ))
            .json(&serde_json::json!({ "urls": chunk }))
            .send()
            .await
        else {
            break;
        };
        if !response.status().is_success() {
            break;
        }
        let Ok(chunk_cached) = response.json::<HashMap<String, String>>().await else {
            break;
        };
        cached.extend(chunk_cached);
    }
    cached
}

pub fn replace_with_cached(
    cached: &HashMap<String, String>,
    processed_image_urls: &mut HashMap<String, String>,
) {
    if cached.is_empty() {
        return;
    }
    for url in processed_image_urls.values_mut() {
        if let Some(direct_url) = cached.get(url) {
            *url = direct_url.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use std::net::TcpListener;

    const MIRROR_URL: &str = "https://images.example.com/mirror/small/dGVzdA==?sig=abc";
    const DIRECT_URL: &str = "https://images.example.com/dGVzdA==_thumbnail_250_750_q80.webp";
    const UNCACHED_MIRROR_URL: &str = "https://images.example.com/mirror/small/b3RoZXI=?sig=abc";

    fn serve_once(body: &'static str) -> (u16, std::thread::JoinHandle<String>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let handle = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut request = Vec::new();
            let mut buffer = [0u8; 1024];
            loop {
                let read = stream.read(&mut buffer).unwrap();
                if read == 0 || {
                    request.extend_from_slice(&buffer[..read]);
                    request.ends_with(b"]}")
                } {
                    break;
                }
            }
            stream
                .write_all(
                    format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {len}\r\n\r\n{body}",
                        len = body.len(),
                    )
                    .as_bytes(),
                )
                .unwrap();
            stream.flush().unwrap();
            String::from_utf8_lossy(&request).to_string()
        });
        (port, handle)
    }

    #[tokio::test]
    async fn cached_images_are_requested_internally_and_replaced_with_direct_urls() {
        let (port, handle) = serve_once(concat!(
            r#"{"https://images.example.com/mirror/small/dGVzdA==?sig=abc":"#,
            r#""https://images.example.com/dGVzdA==_thumbnail_250_750_q80.webp"}"#,
        ));
        init(
            "https://images.example.com/".to_owned(),
            format!("http://127.0.0.1:{port}/"),
            "secret".to_owned(),
        );

        let mut first_post_images = HashMap::from([(
            "https://t.me/i/userpic/320/a.jpg".to_owned(),
            MIRROR_URL.to_owned(),
        )]);
        let mut second_post_images = HashMap::from([(
            "https://t.me/i/userpic/320/b.jpg".to_owned(),
            UNCACHED_MIRROR_URL.to_owned(),
        )]);

        let cached = cached_image_urls([&first_post_images, &second_post_images]).await;
        replace_with_cached(&cached, &mut first_post_images);
        replace_with_cached(&cached, &mut second_post_images);

        assert_eq!(
            signed_image_url("https://t.me/i/userpic/320/a.jpg", ImageVariant::Small),
            format!(
                "https://images.example.com/mirror/small/{base64_url}?sig={signature}",
                base64_url = "aHR0cHM6Ly90Lm1lL2kvdXNlcnBpYy8zMjAvYS5qcGc=",
                signature = "8a20fe3d88611a8a10d254719c87a6646605a1d2fc6928e7c6f4ce04a25ed780",
            )
        );

        let request = handle.join().unwrap();
        assert!(request.starts_with("POST /mirror/cached HTTP/1.1"));

        let body = request.split("\r\n\r\n").nth(1).unwrap();
        let mut sent_urls = serde_json::from_str::<HashMap<String, Vec<String>>>(body)
            .unwrap()
            .remove("urls")
            .unwrap();
        sent_urls.sort();
        assert_eq!(sent_urls, [UNCACHED_MIRROR_URL, MIRROR_URL]);

        assert_eq!(
            first_post_images["https://t.me/i/userpic/320/a.jpg"],
            DIRECT_URL
        );
        assert_eq!(
            second_post_images["https://t.me/i/userpic/320/b.jpg"],
            UNCACHED_MIRROR_URL
        );
    }
}
