pub async fn exec(rb: &rbatis::RBatis) -> Result<(), Box<dyn std::error::Error>> {
    let sql = std::fs::read_to_string("./table_pg.sql")?;
    rb.exec(&sql, vec![]).await?;
    Ok(())
}
