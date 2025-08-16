use translit::{Gost779B, Language, ToLatin};

pub enum TranslitOption {
    ToLowerCase,
    Original,
}

pub struct Transliteration {
    pub transliterated: String,
    pub original: String,
}

pub fn ru_to_latin(values: Vec<String>, case_option: TranslitOption) -> Vec<Transliteration> {
    let transliterator = Gost779B::new(Language::Ru);
    values
        .into_iter()
        .map(|t| {
            let value_for_transliteration = match case_option {
                TranslitOption::Original => t.clone(),
                TranslitOption::ToLowerCase => t.to_lowercase(),
            };
            Transliteration {
                transliterated: transliterator.to_latin(&value_for_transliteration),
                original: t,
            }
        })
        .collect()
}

pub fn ru_to_latin_single(value: String, case_option: TranslitOption) -> Transliteration {
    let value_for_transliteration = match case_option {
        TranslitOption::Original => value.clone(),
        TranslitOption::ToLowerCase => value.to_lowercase(),
    };

    Transliteration {
        transliterated: Gost779B::new(Language::Ru).to_latin(&value_for_transliteration),
        original: value,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ru_to_latin_preserves_original() {
        let values = vec![String::from("Привет")];
        let result = ru_to_latin(values, TranslitOption::Original);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].transliterated, "Privet");
        assert_eq!(result[0].original, "Привет");
    }

    #[test]
    fn ru_to_latin_to_lowercase() {
        let values = vec![String::from("ПрИвЕт")];
        let result = ru_to_latin(values, TranslitOption::ToLowerCase);
        assert_eq!(result[0].transliterated, "privet");
        assert_eq!(result[0].original, "ПрИвЕт");
    }

    #[test]
    fn ru_to_latin_empty_input() {
        let result = ru_to_latin(vec![], TranslitOption::Original);
        assert!(result.is_empty());
    }

    #[test]
    fn ru_to_latin_single_handles_empty_string() {
        let result = ru_to_latin_single(String::new(), TranslitOption::Original);
        assert_eq!(result.transliterated, "");
        assert_eq!(result.original, "");
    }

    #[test]
    fn ru_to_latin_single_to_lowercase() {
        let result = ru_to_latin_single(String::from("ПрИвЕт"), TranslitOption::ToLowerCase);
        assert_eq!(result.transliterated, "privet");
        assert_eq!(result.original, "ПрИвЕт");
    }
}
