mod base;
mod content_formatting;
mod seed_placeholder_ru;

pub async fn exec(rb: &rbatis::RBatis) -> Result<(), Box<dyn std::error::Error>> {
    base::exec(rb).await?;
    content_formatting::exec(rb).await?;
    seed_placeholder_ru::exec(rb).await?;
    Ok(())
}
