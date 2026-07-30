pub fn clean(src: &str) -> String {
    ammonia::Builder::default()
        .add_generic_attributes(&["style"])
        .add_tag_attributes("table", &["border"])
        .add_allowed_classes("img", &["article-img"])
        .add_tags(&["video"])
        .add_tag_attributes("video", &["controls", "autoplay", "loop"])
        .add_allowed_classes("video", &["article-img"])
        .add_tags(&["source"])
        .add_tag_attributes("source", &["src", "type"])
        .add_tags(&["iframe"])
        .add_tag_attributes(
            "iframe",
            &[
                "src",
                "allowfullscreen",
                "width",
                "height",
                "frameBorder",
                "allow",
                "loading",
            ],
        )
        .add_allowed_classes("iframe", &["article-iframe"])
        .clean(src)
        .to_string()
}

pub fn clean_comment(src: &str) -> String {
    ammonia::Builder::default().clean(src).to_string()
}

pub fn to_plain(src: &str) -> String {
    html2text::from_read(src.as_bytes(), usize::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clean_strips_disallowed_tags() {
        let input = "<p>Hello<script>bad()</script><video controls class='article-img' src='v.mp4'></video></p>";
        let cleaned = clean(input);
        assert!(!cleaned.contains("script"));
        assert!(cleaned.contains("<video"));
        assert!(cleaned.contains("article-img"));
    }

    #[test]
    fn clean_comment_keeps_basic_markup() {
        let cleaned = clean_comment("<p>keep <b>bold</b> and <i>italic</i></p>");
        assert_eq!(cleaned, "<p>keep <b>bold</b> and <i>italic</i></p>");
    }

    #[test]
    fn clean_comment_drops_what_clean_allows_for_posts() {
        for input in [
            "<div style=\"background:url(javascript:alert(1))\">x</div>",
            "<iframe src=\"https://example.com\"></iframe>",
            "<video controls src=\"v.mp4\"></video>",
        ] {
            let cleaned = clean_comment(input);
            assert!(!cleaned.contains("style="), "style survived: {cleaned}");
            assert!(!cleaned.contains("<iframe"), "iframe survived: {cleaned}");
            assert!(!cleaned.contains("<video"), "video survived: {cleaned}");
        }
    }

    #[test]
    fn to_plain_removes_html_tags() {
        let input = "<p>Hello <b>World</b></p>";
        let plain = to_plain(input);
        assert!(plain.contains("Hello"));
        assert!(plain.contains("World"));
        assert!(!plain.contains("<"));
    }
}
