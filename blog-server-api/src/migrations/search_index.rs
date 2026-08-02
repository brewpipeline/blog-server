use blog_server_services::traits::post_service::BasePost;
use blog_server_services::utils::time_utils;

const INDEX: &'static str = "idx_post_search_vector";

fn key(config: &str) -> String {
    format!("search_index:{config}")
}

fn vector_expression(config: &str) -> String {
    format!(
        "setweight(to_tsvector('{config}'::regconfig, LOWER(post.title)), 'A') \
         || setweight(to_tsvector('{config}'::regconfig, LOWER(post.summary)), 'B') \
         || setweight(to_tsvector('{config}'::regconfig, LOWER(COALESCE(post.plain_text_content, ''))), 'C')"
    )
}

pub async fn exec(rb: &rbatis::RBatis) -> Result<(), Box<dyn std::error::Error>> {
    let config = BasePost::current_text_search_config();

    let is_indexed: bool = rb
        .query_decode::<u64>(
            "select count(1) as count from migration where key=?",
            vec![rbs::value!(key(config))],
        )
        .await?
        > 0;

    if is_indexed {
        return Ok(());
    }

    rb.exec(&format!("DROP INDEX IF EXISTS {INDEX}"), vec![])
        .await?;
    rb.exec(
        &format!(
            "CREATE INDEX {INDEX} ON post USING GIN (({EXPRESSION}))",
            EXPRESSION = vector_expression(config)
        ),
        vec![],
    )
    .await?;
    rb.exec(
        "delete from migration where key like 'search_index:%'",
        vec![],
    )
    .await?;
    rb.query(
        "insert into migration (key, created_at) values (?, to_timestamp(?))",
        vec![
            rbs::value!(key(config)),
            rbs::value!(time_utils::now_as_secs()),
        ],
    )
    .await?;

    Ok(())
}
