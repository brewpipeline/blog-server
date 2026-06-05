use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::OnceLock;

use base64::engine::general_purpose::URL_SAFE;
use base64::Engine;
use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::utils::map_in_pattern::MapInPattern;

type HmacSha256 = Hmac<Sha256>;

struct ImageSigner {
    processor_url: String,
    secret: String,
}

static IMAGE_SIGNER: OnceLock<ImageSigner> = OnceLock::new();

pub fn init(processor_url: String, secret: String) {
    let _ = IMAGE_SIGNER.set(ImageSigner {
        processor_url,
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
