use translit::{Gost779B, Language, ToLatin};

pub struct Transliteration {
    pub original: String,
    pub transliterated: String,
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
