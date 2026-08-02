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

pub fn to_plain(src: &str) -> String {
    html2text::from_read(src.as_bytes(), usize::MAX)
}

pub fn to_markdown(src: &str) -> String {
    let normalized = normalize_inline_tags(src);
    html2text::from_read_with_decorator(
        normalized.as_bytes(),
        MARKDOWN_WIDTH,
        MarkdownDecorator::new(),
    )
    .replace(STRIKEOUT_OVERLAY, "")
    .trim()
    .to_string()
}

const MARKDOWN_WIDTH: usize = 120;
const STRIKEOUT_OVERLAY: char = '\u{336}';

const INLINE_TAG_ALIASES: &[(&str, &str)] =
    &[("b", "strong"), ("i", "em"), ("del", "s"), ("strike", "s")];

fn normalize_inline_tags(src: &str) -> String {
    INLINE_TAG_ALIASES
        .iter()
        .fold(src.to_string(), |acc, (from, to)| {
            rename_tag(&acc, from, to)
        })
}

fn rename_tag(src: &str, from: &str, to: &str) -> String {
    let mut out = String::with_capacity(src.len());
    let mut rest = src;

    while let Some(index) = rest.find('<') {
        out.push_str(&rest[..index]);
        rest = &rest[index..];

        let name_start = if rest.starts_with("</") { 2 } else { 1 };
        let after_bracket = &rest[name_start..];
        let name_len = after_bracket
            .find(|c: char| !c.is_ascii_alphanumeric())
            .unwrap_or(after_bracket.len());

        if after_bracket[..name_len].eq_ignore_ascii_case(from) {
            out.push_str(&rest[..name_start]);
            out.push_str(to);
            rest = &after_bracket[name_len..];
        } else {
            out.push_str(&rest[..name_start]);
            rest = after_bracket;
        }
    }

    out.push_str(rest);
    out
}

#[derive(Clone, Debug, Default)]
struct MarkdownDecorator {
    link_urls: Vec<String>,
}

impl MarkdownDecorator {
    fn new() -> Self {
        Self::default()
    }
}

impl html2text::render::text_renderer::TextDecorator for MarkdownDecorator {
    type Annotation = ();

    fn decorate_link_start(&mut self, url: &str) -> (String, Self::Annotation) {
        self.link_urls.push(url.to_string());
        ("[".to_string(), ())
    }

    fn decorate_link_end(&mut self) -> String {
        let url = self.link_urls.pop().unwrap_or_default();
        format!("]({url})")
    }

    fn decorate_em_start(&mut self) -> (String, Self::Annotation) {
        ("*".to_string(), ())
    }

    fn decorate_em_end(&mut self) -> String {
        "*".to_string()
    }

    fn decorate_strong_start(&mut self) -> (String, Self::Annotation) {
        ("**".to_string(), ())
    }

    fn decorate_strong_end(&mut self) -> String {
        "**".to_string()
    }

    fn decorate_strikeout_start(&mut self) -> (String, Self::Annotation) {
        ("~~".to_string(), ())
    }

    fn decorate_strikeout_end(&mut self) -> String {
        "~~".to_string()
    }

    fn decorate_code_start(&mut self) -> (String, Self::Annotation) {
        ("`".to_string(), ())
    }

    fn decorate_code_end(&mut self) -> String {
        "`".to_string()
    }

    fn decorate_preformat_first(&mut self) -> Self::Annotation {}

    fn decorate_preformat_cont(&mut self) -> Self::Annotation {}

    fn decorate_image(&mut self, src: &str, title: &str) -> (String, Self::Annotation) {
        (format!("![{title}]({src})"), ())
    }

    fn header_prefix(&mut self, level: usize) -> String {
        "#".repeat(level) + " "
    }

    fn quote_prefix(&mut self) -> String {
        "> ".to_string()
    }

    fn unordered_item_prefix(&mut self) -> String {
        "- ".to_string()
    }

    fn ordered_item_prefix(&mut self, i: i64) -> String {
        format!("{i}. ")
    }

    fn make_subblock_decorator(&self) -> Self {
        self.clone()
    }

    fn finalise(
        &mut self,
        _links: Vec<String>,
    ) -> Vec<html2text::render::text_renderer::TaggedLine<Self::Annotation>> {
        Vec::new()
    }
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
    fn to_plain_removes_html_tags() {
        let input = "<p>Hello <b>World</b></p>";
        let plain = to_plain(input);
        assert!(plain.contains("Hello"));
        assert!(plain.contains("World"));
        assert!(!plain.contains("<"));
    }

    #[test]
    fn to_markdown_keeps_inline_links() {
        let markdown = to_markdown("<p>see <a href=\"https://example.com/a\">this</a></p>");
        assert!(markdown.contains("[this](https://example.com/a)"));
    }

    #[test]
    fn to_markdown_renders_headers_and_lists() {
        let markdown = to_markdown("<h2>Title</h2><ul><li>one</li><li>two</li></ul>");
        assert!(markdown.contains("## Title"));
        assert!(markdown.contains("- one"));
        assert!(markdown.contains("- two"));
    }

    #[test]
    fn to_markdown_renders_emphasis_and_images() {
        let markdown = to_markdown(
            "<p><strong>bold</strong> <em>italic</em></p><img src=\"/i.png\" alt=\"x\">",
        );
        assert!(markdown.contains("**bold**"));
        assert!(markdown.contains("*italic*"));
        assert!(markdown.contains("![x](/i.png)"));
    }

    #[test]
    fn to_markdown_understands_editor_inline_tags() {
        let markdown = to_markdown(
            "<p><b style=\"x\">bold</b> <i>italic</i> <del>gone</del> <strike>gone too</strike></p>",
        );
        assert!(markdown.contains("**bold**"));
        assert!(markdown.contains("*italic*"));
        assert!(markdown.contains("~~gone~~"));
        assert!(markdown.contains("~~gone too~~"));
    }

    #[test]
    fn rename_tag_leaves_similar_names_alone() {
        let renamed = rename_tag("<b>x</b><body>y</body><br/>", "b", "strong");
        assert_eq!(renamed, "<strong>x</strong><body>y</body><br/>");
    }
}
