#![allow(dead_code)]

mod endpoints;
mod extensions;
mod router;
mod utils;

#[cfg(feature = "ssr")]
const SITE_URL: &'static str = std::env!("SITE_URL"); // http://127.0.0.1:3000

const JWT_SECRET: &'static str = std::env!("JWT_SECRET"); // 123
const SERVER_ADDRESS: &'static str = std::env!("SERVER_ADDRESS"); // 127.0.0.1:3000
const PG_URL: &'static str = std::env!("PG_URL"); // postgres://postgres:postgres@localhost:5432/blog
const RABBIT_URL: &'static str = std::env!("RABBIT_URL"); // amqp://guest:guest@localhost:5672/
const TELEGRAM_BOT_TOKEN: &'static str = std::env!("TELEGRAM_BOT_TOKEN"); // XXXXXXXX:XXXXXXXXXXXXXXXXXXXXXXXX

#[tokio::main]
async fn main() -> screw_components::dyn_result::DResult<()> {
    let rbatis = init_db().await;

    let rabbit = init_rabbit().await;

    let server_service = screw_core::server::ServerService::with_responder_factory(
        screw_core::responder_factory::ResponderFactory::with_router(router::make_router())
            .and_extensions(extensions::make_extensions(rbatis, rabbit)),
    );

    let addr = SERVER_ADDRESS.parse()?;
    println!("Listening on http://{}", addr);
    hyper::Server::bind(&addr).serve(server_service).await?;

    Ok(())
}

pub async fn init_db() -> rbatis::RBatis {
    let rb = rbatis::RBatis::new();
    rb.init(rbdc_pg::driver::PgDriver {}, PG_URL)
        .expect("DB init failed");
    migrate_db(&rb).await.expect("DB migration failed");
    return rb;
}

async fn migrate_db(rb: &rbatis::RBatis) -> Result<(), Box<dyn std::error::Error>> {
    let sql = std::fs::read_to_string("./table_pg.sql")?;
    rb.exec(&sql, vec![]).await?;

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

    Ok(())
}

pub async fn init_rabbit(
) -> Box<dyn blog_server_services::traits::event_bus_service::EventBusService> {
    if RABBIT_URL.is_empty() {
        blog_server_services::impls::create_rabbit_event_bus_service(None).await
    } else {
        blog_server_services::impls::create_rabbit_event_bus_service(Some(RABBIT_URL)).await
    }
}
