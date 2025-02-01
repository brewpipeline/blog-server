pub async fn exec(rb: &rbatis::RBatis) -> Result<(), Box<dyn std::error::Error>> {
    let sql = include_str!("table_pg.sql");
    rb.exec(&sql, vec![]).await?;
    Ok(())
}
