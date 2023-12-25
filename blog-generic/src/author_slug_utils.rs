pub const SPLIT_SYMBOL: char = '^';

pub fn clean(slug: &String) -> String {
    slug.rsplit_once(SPLIT_SYMBOL)
        .map(|r| r.0.to_owned())
        .unwrap_or(slug.clone())
}

pub fn extend(slug: &String, suffix: &String) -> String {
    format!("{slug}{SPLIT_SYMBOL}{suffix}")
}
