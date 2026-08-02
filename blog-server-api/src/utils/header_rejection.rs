use std::fmt::Display;

pub struct HeaderRejection(pub String);

impl HeaderRejection {
    pub fn missing(name: &'static str) -> Self {
        Self(format!("{name} header is missing or empty"))
    }

    pub fn unparsed(name: &'static str, error: impl Display) -> Self {
        Self(format!("{name} header is not valid: {error}"))
    }
}
