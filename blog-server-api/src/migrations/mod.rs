mod base;
mod content;

pub async fn exec(rb: &rbatis::RBatis) -> Result<(), Box<dyn std::error::Error>> {
    base::exec(rb).await?;
    content::exec(rb).await?;
    Ok(())
}
