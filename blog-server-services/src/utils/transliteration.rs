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
