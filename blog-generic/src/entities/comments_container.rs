use super::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentsContainer {
    pub comments: Vec<Comment>,
    #[serde(flatten)]
    pub base: TotalOffsetLimitContainer,
}
