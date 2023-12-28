const KEY: &'static str = "content_formatting_migration_status";

pub async fn exec(rb: &rbatis::RBatis) -> Result<(), Box<dyn std::error::Error>> {
    let is_content_migrated: bool = rb
        .query_decode::<Option<String>>(
            "select value from property where key=?",
            vec![rbs::to_value!(KEY)],
        )
        .await?
        .map(|v| v == "ok")
        .unwrap_or_default();

    if !is_content_migrated {
        let posts: Vec<blog_server_services::traits::post_service::Post> =
            rb.query_decode("select * from post", vec![]).await?;
        for post in posts {
            let content = post
                .base
                .content
                .as_ref()
                .map(|c| blog_server_services::utils::html::clean(c));
            let plain_text_content = content
                .as_ref()
                .map(|c| blog_server_services::utils::html::to_plain(c));
            rb.query(
                "update post set content=?, plain_text_content=? where id=?",
                vec![
                    rbs::to_value!(content),
                    rbs::to_value!(plain_text_content),
                    rbs::to_value!(post.id),
                ],
            )
            .await?;
        }
        rb.query(
            "insert into property (key, value) values (?, 'ok')",
            vec![rbs::to_value!(KEY)],
        )
        .await?;
    }

    Ok(())
}
