#![allow(dead_code)]
#![allow(unused_imports)]

mod endpoints;
mod extensions;
mod migrations;
mod router;
mod utils;

#[macro_use]
extern crate async_trait;

const SITE_URL: &'static str = env!("SITE_URL"); // http://127.0.0.1:3000
const JWT_SECRET: &'static str = env!("JWT_SECRET"); // 123
const SERVER_ADDRESS: &'static str = env!("SERVER_ADDRESS"); // 127.0.0.1:3000
const PG_URL: &'static str = env!("PG_URL"); // postgres://postgres:postgres@localhost:5432/blog
const RABBIT_URL: &'static str = env!("RABBIT_URL"); // amqp://guest:guest@localhost:5672/
const TELEGRAM_BOT_TOKEN: &'static str = env!("TELEGRAM_BOT_TOKEN"); // XXXXXXXX:XXXXXXXXXXXXXXXXXXXXXXXX
const OPENAI_API_KEY: &'static str = env!("OPENAI_API_KEY"); // OpenAI API key

#[tokio::main]
async fn main() -> screw_components::dyn_result::DResult<()> {
    let config = init_config().await;
    let rbatis = init_db().await;

    let rabbit_event_bus_service =
        match blog_server_services::impls::create_rabbit_event_bus_service(RABBIT_URL).await {
            Ok(rabbit_event_bus_service) => Some(rabbit_event_bus_service),
            Err(err) => {
                println!("Error while connecting to rabbitMQ: {err}");
                None
            }
        };

    let server_service = screw_core::server::ServerService::with_responder_factory(
        screw_core::responder_factory::ResponderFactory::with_router(router::make_router())
            .and_extensions(extensions::make_extensions(
                config,
                rbatis,
                rabbit_event_bus_service,
            )),
    );

    let addr = SERVER_ADDRESS.parse()?;
    println!("Listening on http://{}", addr);
    hyper::Server::bind(&addr).serve(server_service).await?;

    Ok(())
}

pub async fn init_config() -> config::Config {
    config::Config::builder()
        .add_source(config::File::with_name("config"))
        .build()
        .expect("CONFIG init failed")
}

pub async fn init_db() -> rbatis::RBatis {
    let rb = rbatis::RBatis::new();
    rb.init(rbdc_pg::driver::PgDriver {}, PG_URL)
        .expect("DB init failed");
    migrations::exec(&rb).await.expect("DB migration failed");
    rb
}
