use std::time::{SystemTime, UNIX_EPOCH};

pub fn now_as_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
