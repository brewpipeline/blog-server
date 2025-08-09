pub const SPLIT_SYMBOL: char = '^';

pub fn clean(slug: &String) -> String {
    slug.rsplit_once(SPLIT_SYMBOL)
        .map(|r| r.0.to_owned())
        .unwrap_or(slug.clone())
}

pub fn extend(slug: &String, suffix: &String) -> String {
    format!("{slug}{SPLIT_SYMBOL}{suffix}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clean_removes_suffix() {
        let slug = String::from("author^extra");
        assert_eq!(clean(&slug), "author");
    }

    #[test]
    fn clean_returns_original_when_no_symbol() {
        let slug = String::from("author");
        assert_eq!(clean(&slug), "author");
    }

    #[test]
    fn clean_handles_multiple_symbols() {
        let slug = String::from("author^mid^end");
        assert_eq!(clean(&slug), "author^mid");
    }

    #[test]
    fn clean_handles_empty_string() {
        let slug = String::new();
        assert_eq!(clean(&slug), "");
    }

    #[test]
    fn extend_joins_slug_and_suffix() {
        let slug = String::from("author");
        let suffix = String::from("meta");
        assert_eq!(extend(&slug, &suffix), "author^meta");
    }

    #[test]
    fn extend_allows_empty_parts() {
        assert_eq!(extend(&String::new(), &String::from("meta")), "^meta");
        assert_eq!(extend(&String::from("author"), &String::new()), "author^");
    }
}
