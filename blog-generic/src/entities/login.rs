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
pub struct LoginTelegramQuestion {
    #[serde(alias = "id")]
    pub id: u64,
    #[serde(alias = "first_name")]
    pub first_name: String,
    #[serde(alias = "last_name")]
    pub last_name: String,
    #[serde(alias = "username")]
    pub username: Option<String>,
    #[serde(alias = "photo_url")]
    pub photo_url: Option<String>,
    #[serde(alias = "auth_date")]
    pub auth_date: u64,
    #[serde(alias = "hash")]
    pub hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginAnswer {
    pub token: String,
}
