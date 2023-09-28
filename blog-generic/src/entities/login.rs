use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginQuestion {
    pub slug: String,
    pub password: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginYandexQuestion {
    #[serde(alias = "access_token")]
    pub access_token: String,
    #[serde(alias = "token_type")]
    pub token_type: String,
    #[serde(alias = "expires_in")]
    pub expires_in: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginAnswer {
    pub token: String,
}
