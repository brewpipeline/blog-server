pub trait MapInPattern {
    fn map_in_pattern<F>(self, pattern: [&str; 2], transform: F) -> String
    where
        F: Fn(&str) -> String;
}

impl<T: AsRef<str>> MapInPattern for T {
    fn map_in_pattern<F>(self, pattern: [&str; 2], transform: F) -> String
    where
        F: Fn(&str) -> String,
    {
        let str: &str = self.as_ref();
        let mut result = String::new();
        let mut start = 0;

        while let Some(start_index) = str[start..].find(pattern[0]) {
            let inner_start = start + start_index + pattern[0].len();

            let Some(inner_len) = str[inner_start..].find(pattern[1]) else {
                break;
            };

            let inner_end = inner_start + inner_len;
            let end = inner_end + pattern[1].len();

            result.push_str(&str[start..inner_start]);
            result.push_str(&transform(&str[inner_start..inner_end]));
            result.push_str(&str[inner_end..end]);

            start = end;
        }

        result.push_str(&str[start..]);
        result
    }
}
