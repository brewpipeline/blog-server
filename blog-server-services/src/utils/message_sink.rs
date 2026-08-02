#[async_trait]
pub trait MessageSink: Send + Sync {
    async fn send(&self, message: &str);
}
