use std::fmt::Write;
use std::sync::Arc;

use blog_generic::entities::*;
use blog_generic::{DefaultPageProcessor, PageProcessor};
use blog_server_services::traits::author_service::AuthorService;
use blog_server_services::traits::entity_post_service::EntityPostService;
use blog_server_services::traits::post_service::PostService;
use blog_server_services::utils::html;
use blog_ui::Route;

use crate::endpoints::*;
use crate::extensions::Resolve;

use screw_core::request::*;
use screw_core::routing::*;

pub const MARKDOWN_CONTENT_TYPE: &str = "text/markdown; charset=utf-8";

pub async fn markdown_page<Extensions>(
    request: &router::RoutedRequest<Request<Extensions>>,
) -> Option<(hyper::StatusCode, String)>
where
    Extensions: Resolve<Arc<dyn AuthorService>>
        + Resolve<Arc<dyn PostService>>
        + Resolve<Arc<dyn EntityPostService>>,
{
    let page = request
        .query
        .get("page")
        .and_then(|v| v.parse().ok())
        .unwrap_or(1);
    let page_processor: DefaultPageProcessor = PageProcessor::create_for_page(&page);
    let ext = &request.origin.extensions;

    match Route::recognize_path(request.path.as_str())? {
        Route::Post { slug, id } => {
            let container = post::direct_handler(id.to_string(), ext.resolve(), ext.resolve())
                .await
                .filter(|c| c.post.id == id && c.post.slug == slug)?;
            Some((hyper::StatusCode::OK, post_markdown(&container.post)))
        }
        Route::Posts => {
            let container = posts::direct_handler(
                page_processor.offset(),
                page_processor.limit(),
                ext.resolve(),
                ext.resolve(),
            )
            .await?;
            let status = page_status(!container.posts.is_empty());
            Some((status, posts_markdown(&container)))
        }
        Route::Author { slug } => {
            let container = author::direct_handler(slug, ext.resolve()).await?;
            Some((hyper::StatusCode::OK, author_markdown(&container.author)))
        }
        Route::Authors => {
            let container = authors::direct_handler(
                page_processor.offset(),
                page_processor.limit(),
                ext.resolve(),
            )
            .await?;
            let status = page_status(!container.authors.is_empty());
            Some((status, authors_markdown(&container)))
        }
        _ => None,
    }
}

fn page_status(found: bool) -> hyper::StatusCode {
    if found {
        hyper::StatusCode::OK
    } else {
        hyper::StatusCode::NOT_FOUND
    }
}

fn post_markdown(post: &Post) -> String {
    let mut out = String::new();
    let mut front = FrontMatter::new();
    front.text("type", "post");
    front.text("title", &post.title);
    front.text("url", &post_url(post));
    front.text("author", &author_name(&post.author));
    front.text("authorUrl", &author_url(&post.author));
    front.text("published", &rfc3339(post.created_at));
    if !post.tags.is_empty() {
        front.list("tags", post.tags.iter().map(|t| t.title.as_str()));
    }
    front.write_to(&mut out);

    let _ = writeln!(out, "# {}\n", post.title);
    let _ = writeln!(out, "> {}\n", post.summary.replace('\n', " "));
    if let Some(content) = post.content.as_deref() {
        let _ = writeln!(out, "{}", html::to_markdown(content));
    }
    out
}

fn posts_markdown(container: &PostsContainer) -> String {
    let mut out = String::new();
    let mut front = FrontMatter::new();
    front.text("type", "posts");
    front.text("url", &format!("{site_url}/", site_url = &*crate::SITE_URL));
    front.number("total", container.base.total);
    front.number("offset", container.base.offset);
    front.number("limit", container.base.limit);
    front.write_to(&mut out);

    for post in &container.posts {
        let _ = writeln!(out, "## [{}]({})\n", post.title, post_url(post));
        let _ = writeln!(out, "{}\n", post.summary.replace('\n', " "));
        let mut facts = vec![
            rfc3339(post.created_at),
            format!(
                "[{}]({})",
                author_name(&post.author),
                author_url(&post.author)
            ),
        ];
        if !post.tags.is_empty() {
            facts.push(post.joined_tags_string(", "));
        }
        let _ = writeln!(out, "{}\n", facts.join(" · "));
    }
    out
}

fn author_markdown(author: &Author) -> String {
    let mut out = String::new();
    let mut front = FrontMatter::new();
    front.text("type", "author");
    front.text("name", &author_name(author));
    front.text("url", &author_url(author));
    front.text("registered", &rfc3339(author.registered_at));
    front.write_to(&mut out);

    let _ = writeln!(out, "# {}\n", author_name(author));
    if let Some(status) = author.status.as_deref().filter(|s| !s.is_empty()) {
        let _ = writeln!(out, "{status}\n");
    }
    out
}

fn authors_markdown(container: &AuthorsContainer) -> String {
    let mut out = String::new();
    let mut front = FrontMatter::new();
    front.text("type", "authors");
    front.text(
        "url",
        &format!("{site_url}/authors", site_url = &*crate::SITE_URL),
    );
    front.number("total", container.base.total);
    front.number("offset", container.base.offset);
    front.number("limit", container.base.limit);
    front.write_to(&mut out);

    for author in &container.authors {
        let _ = writeln!(
            out,
            "## [{}]({})\n",
            author_name(author),
            author_url(author)
        );
        if let Some(status) = author.status.as_deref().filter(|s| !s.is_empty()) {
            let _ = writeln!(out, "{status}\n");
        }
    }
    out
}

fn post_url(post: &Post) -> String {
    format!(
        "{site_url}/post/{slug}/{id}",
        site_url = &*crate::SITE_URL,
        slug = post.slug,
        id = post.id,
    )
}

fn author_url(author: &Author) -> String {
    format!(
        "{site_url}/author/{slug}",
        site_url = &*crate::SITE_URL,
        slug = author.slug,
    )
}

fn author_name(author: &Author) -> String {
    let name = [
        author.first_name.as_deref(),
        author.middle_name.as_deref(),
        author.last_name.as_deref(),
    ]
    .into_iter()
    .flatten()
    .filter(|part| !part.is_empty())
    .collect::<Vec<&str>>()
    .join(" ");

    if name.is_empty() {
        author.slug.clone()
    } else {
        name
    }
}

fn rfc3339(milliseconds: u64) -> String {
    chrono::DateTime::from_timestamp(milliseconds as i64 / 1000, 0)
        .unwrap_or_default()
        .to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
}

struct FrontMatter {
    lines: Vec<String>,
}

impl FrontMatter {
    fn new() -> Self {
        Self { lines: Vec::new() }
    }

    fn text(&mut self, key: &str, value: &str) {
        self.lines.push(format!("{key}: {}", quoted(value)));
    }

    fn number(&mut self, key: &str, value: u64) {
        self.lines.push(format!("{key}: {value}"));
    }

    fn list<'a>(&mut self, key: &str, values: impl Iterator<Item = &'a str>) {
        let values = values.map(quoted).collect::<Vec<String>>().join(", ");
        self.lines.push(format!("{key}: [{values}]"));
    }

    fn write_to(self, out: &mut String) {
        let _ = writeln!(out, "---");
        for line in self.lines {
            let _ = writeln!(out, "{line}");
        }
        let _ = writeln!(out, "---\n");
    }
}

fn quoted(value: &str) -> String {
    let escaped = value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace(['\n', '\r'], " ");
    format!("\"{escaped}\"")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_site_url() {
        static ONCE: std::sync::Once = std::sync::Once::new();
        ONCE.call_once(|| unsafe { std::env::set_var("SITE_URL", "https://example.com") });
    }

    fn author() -> Author {
        Author {
            id: 1,
            slug: "jane_doe".to_string(),
            first_name: Some("Jane".to_string()),
            middle_name: None,
            last_name: Some("Doe".to_string()),
            mobile: None,
            email: None,
            registered_at: 1_700_000_000_000,
            status: Some("writes things".to_string()),
            image_url: None,
            processed_image_urls: Default::default(),
            editor: 0,
            blocked: 0,
            notification_subscribed: None,
            override_social_data: 0,
        }
    }

    fn post() -> Post {
        Post {
            id: 7,
            title: "A \"quoted\" title".to_string(),
            slug: "a-quoted-title".to_string(),
            summary: "Summary of the post.".to_string(),
            publish_type: PublishType::Published,
            recommended: false,
            created_at: 1_700_000_000_000,
            content: Some("<p>Hello <b>world</b></p>".to_string()),
            author: author(),
            tags: vec![Tag {
                id: 3,
                title: "rust".to_string(),
                slug: "rust".to_string(),
            }],
            image_url: None,
            processed_image_urls: Default::default(),
            noindex: false,
        }
    }

    #[test]
    fn post_markdown_has_front_matter_and_content() {
        init_site_url();
        let markdown = post_markdown(&post());

        assert!(markdown.starts_with("---\n"));
        assert!(markdown.contains("type: \"post\"\n"));
        assert!(markdown.contains("title: \"A \\\"quoted\\\" title\"\n"));
        assert!(markdown.contains("url: \"https://example.com/post/a-quoted-title/7\"\n"));
        assert!(markdown.contains("author: \"Jane Doe\"\n"));
        assert!(markdown.contains("published: \"2023-11-14T22:13:20Z\"\n"));
        assert!(markdown.contains("tags: [\"rust\"]\n"));
        assert!(markdown.contains("# A \"quoted\" title"));
        assert!(markdown.contains("> Summary of the post."));
        assert!(markdown.contains("Hello **world**"));
    }

    #[test]
    fn posts_markdown_lists_every_post() {
        init_site_url();
        let container = PostsContainer {
            posts: vec![post()],
            base: TotalOffsetLimitContainer {
                total: 1,
                offset: 0,
                limit: 10,
            },
        };
        let markdown = posts_markdown(&container);

        assert!(markdown.contains("type: \"posts\"\n"));
        assert!(markdown.contains("total: 1\n"));
        assert!(
            markdown.contains("## [A \"quoted\" title](https://example.com/post/a-quoted-title/7)")
        );
        assert!(markdown.contains("[Jane Doe](https://example.com/author/jane_doe)"));
    }

    #[test]
    fn author_name_falls_back_to_slug() {
        let mut author = author();
        author.first_name = None;
        author.last_name = None;
        assert_eq!(author_name(&author), "jane_doe");
    }
}
