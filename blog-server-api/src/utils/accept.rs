use hyper::header::{ACCEPT, HeaderMap};

pub const MARKDOWN_MEDIA_TYPE: &str = "text/markdown";
pub const HTML_MEDIA_TYPE: &str = "text/html";

pub fn prefers_markdown(headers: &HeaderMap) -> bool {
    let Some(accept) = headers.get(ACCEPT).and_then(|v| v.to_str().ok()) else {
        return false;
    };
    quality(accept, MARKDOWN_MEDIA_TYPE) > quality(accept, HTML_MEDIA_TYPE)
}

fn quality(accept: &str, media_type: &str) -> f32 {
    let (media_kind, _) = media_type.split_once('/').unwrap_or((media_type, ""));

    let mut best = None;
    let mut best_precision = 0;

    for entry in accept.split(',') {
        let mut parts = entry.split(';').map(str::trim);
        let Some(candidate) = parts.next() else {
            continue;
        };

        let precision = if candidate.eq_ignore_ascii_case(media_type) {
            3
        } else if candidate.eq_ignore_ascii_case(&format!("{media_kind}/*")) {
            2
        } else if candidate == "*/*" {
            1
        } else {
            continue;
        };

        if precision < best_precision {
            continue;
        }

        let q = parts
            .find_map(|p| p.strip_prefix("q="))
            .and_then(|q| q.parse::<f32>().ok())
            .unwrap_or(1.0);

        if precision > best_precision || q > best.unwrap_or(0.0) {
            best_precision = precision;
            best = Some(q);
        }
    }

    best.unwrap_or(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn headers(accept: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, accept.parse().unwrap());
        headers
    }

    #[test]
    fn browser_accept_keeps_html() {
        assert!(!prefers_markdown(&headers(
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8"
        )));
    }

    #[test]
    fn explicit_markdown_wins() {
        assert!(prefers_markdown(&headers("text/markdown")));
        assert!(prefers_markdown(&headers("text/markdown, text/html;q=0.9")));
        assert!(prefers_markdown(&headers(
            "text/html;q=0.5, text/markdown;q=1.0"
        )));
    }

    #[test]
    fn html_wins_when_ranked_higher() {
        assert!(!prefers_markdown(&headers(
            "text/markdown;q=0.5, text/html;q=0.9"
        )));
    }

    #[test]
    fn wildcards_do_not_prefer_markdown() {
        assert!(!prefers_markdown(&headers("*/*")));
        assert!(!prefers_markdown(&headers("text/*")));
    }

    #[test]
    fn missing_or_unrelated_accept_keeps_html() {
        assert!(!prefers_markdown(&HeaderMap::new()));
        assert!(!prefers_markdown(&headers("application/json")));
    }

    #[test]
    fn wildcard_covers_html_when_markdown_is_downranked() {
        assert!(!prefers_markdown(&headers(
            "text/markdown;q=0.6, */*;q=0.9"
        )));
        assert!(prefers_markdown(&headers("text/markdown;q=0.9, */*;q=0.6")));
    }
}
