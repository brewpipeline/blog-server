pub mod entities;
pub mod events;

pub const AUTHOR_SLUG_SPLIT_SYMBOL: char = '^';

pub fn clean_author_slug(slug: &String) -> String {
    slug.rsplit_once(AUTHOR_SLUG_SPLIT_SYMBOL)
        .map(|r| r.0.to_owned())
        .unwrap_or(slug.clone())
}

pub fn extend_author_slug(slug: &String, suffix: &String) -> String {
    format!("{slug}{AUTHOR_SLUG_SPLIT_SYMBOL}{suffix}")
}

pub const ITEMS_PER_PAGE: u64 = 10;

pub fn offset_for_page<const LIMIT: u64>(page: &u64) -> u64 {
    let Some(real_page) = page.checked_sub(1) else {
        return 0;
    };
    let offset = real_page * LIMIT;
    offset
}
