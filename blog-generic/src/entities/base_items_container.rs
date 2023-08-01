use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseItemsContainer {
    pub total: u64,
    pub offset: u64,
    pub limit: u64,
}
