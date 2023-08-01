use super::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    #[serde(flatten)]
    pub base: ShortAuthor,
    pub middle_name: Option<String>,
    pub mobile: Option<String>,
    pub email: Option<String>,
    pub registered_at: i64,
    pub status: Option<String>,
}
