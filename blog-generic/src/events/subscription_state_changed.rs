use serde::Serialize;

#[derive(Serialize)]
pub struct SubscriptionStateChanged {
    pub blog_user_id: u64,
    pub user_telegram_id: u64,
    pub new_state: u8,
}
