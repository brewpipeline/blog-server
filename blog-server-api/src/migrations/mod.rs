mod base;
mod content_formatting;

pub async fn exec(rb: &rbatis::RBatis) -> Result<(), Box<dyn std::error::Error>> {
    base::exec(rb).await?;
    content_formatting::exec(rb).await?;
    Ok(())
}
