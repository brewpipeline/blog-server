use translit::{Gost779B, Language, ToLatin};

pub struct Transliteration {
    pub transliterated: String,
    pub original: String,
}

pub fn ru_to_latin(values: Vec<String>) -> Vec<Transliteration> {
    let transliterator = Gost779B::new(Language::Ru);
    values
        .into_iter()
        .map(|t| Transliteration {
            transliterated: transliterator.to_latin(&t),
            original: t,
        })
        .collect()
}

pub fn ru_to_latin_single(value: String) -> Transliteration {
    Transliteration {
        transliterated: Gost779B::new(Language::Ru).to_latin(&value),
        original: value,
    }
}
