use super::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostsContainer {
    pub posts: Vec<Post>,
    #[serde(flatten)]
    pub base: TotalOffsetLimitContainer,
}
