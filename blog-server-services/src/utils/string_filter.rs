pub fn remove_non_latin_or_number_chars(input_string: &str) -> String {
    let mut previous_symbol_is_whitespace = false;
    let filtered = input_string
        .trim()
        .chars()
        .fold(String::new(), |mut result, c| {
            match c {
                c if c.is_ascii_alphanumeric() => {
                    result.push(c);
                    previous_symbol_is_whitespace = false;
                }
                c if c.is_whitespace() => {
                    if !previous_symbol_is_whitespace {
                        result.push(c);
                        previous_symbol_is_whitespace = true;
                    }
                }
                _ => (),
            };
            result
        });
    filtered.trim().to_string()
}

#[cfg(test)]
mod tests {
    use crate::utils::string_filter::remove_non_latin_or_number_chars;

    #[test]
    fn should_remove_non_latin_or_numeric_chars() {
        let original = String::from("Test1 Привет !something'");
        assert_eq!(
            remove_non_latin_or_number_chars(&original),
            "Test1 something"
        )
    }

    #[test]
    fn should_remove_subsequent_whitespaces() {
        let original = String::from("first        second");
        assert_eq!(remove_non_latin_or_number_chars(&original), "first second")
    }

    #[test]
    fn should_trim_leading_and_trailing_whitespaces() {
        let original = String::from("    Test     ");
        assert_eq!(remove_non_latin_or_number_chars(&original), "Test")
    }

    #[test]
    fn should_return_empty_string_if_all_symbols_are_forbidden() {
        let original = String::from("Ну да,Ну да....");
        assert_eq!(remove_non_latin_or_number_chars(&original), "")
    }
}
