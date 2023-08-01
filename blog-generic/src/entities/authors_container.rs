use super::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorsContainer {
    pub authors: Vec<Author>,
    #[serde(flatten)]
    pub base: TotalOffsetLimitContainer,
}
