pub fn clean(src: &str) -> String {
    ammonia::Builder::default()
        .add_generic_attributes(&["style"])
        .add_tag_attributes("table", &["border"])
        .add_tags(&["iframe"])
        .add_tag_attributes("iframe", &["src", "allowfullscreen"])
        .add_allowed_classes("img", &["article-img"])
        .add_allowed_classes("iframe", &["article-iframe"])
        .clean(src)
        .to_string()
}
