pub mod entities;

const AUTHOR_SLUG_SPLIT_SYMBOL: char = '^';

pub fn clean_author_slug(slug: &String) -> String {
    slug.rsplit_once(AUTHOR_SLUG_SPLIT_SYMBOL)
        .map(|r| r.0.to_owned())
        .unwrap_or(slug.clone())
}

pub fn extend_author_slug(slug: &String, suffix: &String) -> String {
    format!("{slug}{AUTHOR_SLUG_SPLIT_SYMBOL}{suffix}")
}
