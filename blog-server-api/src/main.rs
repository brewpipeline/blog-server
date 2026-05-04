#![allow(dead_code)]
#![allow(unused_imports)]

mod endpoints;
mod extensions;
mod migrations;
mod router;
mod utils;

#[macro_use]
extern crate async_trait;

use once_cell::sync::Lazy;

pub(crate) static JWT_SECRET: Lazy<String> =
    Lazy::new(|| std::env::var("JWT_SECRET").expect("JWT_SECRET not set"));
pub(crate) static SITE_URL: Lazy<String> =
    Lazy::new(|| std::env::var("SITE_URL").expect("SITE_URL not set"));
pub(crate) static PG_URL: Lazy<String> =
    Lazy::new(|| std::env::var("PG_URL").expect("PG_URL not set"));
pub(crate) static RABBIT_URL: Lazy<String> =
    Lazy::new(|| std::env::var("RABBIT_URL").expect("RABBIT_URL not set"));
pub(crate) static TELEGRAM_BOT_TOKEN: Lazy<String> =
    Lazy::new(|| std::env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set"));
pub(crate) static SERVER_ADDRESS: Lazy<String> =
    Lazy::new(|| std::env::var("SERVER_ADDRESS").expect("SERVER_ADDRESS not set"));
#[cfg(feature = "chatgpt")]
pub(crate) static OPENAI_API_KEY: Lazy<String> =
    Lazy::new(|| std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set"));

#[tokio::main]
async fn main() -> screw_components::dyn_result::DResult<()> {
    let config = init_config().await;
    let rbatis = init_db().await;

    let rabbit_event_bus_service =
        match blog_server_services::impls::create_rabbit_event_bus_service(&*RABBIT_URL).await {
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

    let addr: std::net::SocketAddr = SERVER_ADDRESS.parse()?;
    println!("Listening on http://{}", addr);

    let server_service = std::sync::Arc::new(server_service);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    loop {
        let (stream, remote_addr) = listener.accept().await?;
        let session_service = server_service.make_session_service(remote_addr);
        tokio::task::spawn(serve_connection(stream, session_service));
    }
}

async fn serve_connection(
    stream: tokio::net::TcpStream,
    session_service: impl hyper::service::Service<
        hyper::Request<hyper::body::Incoming>,
        Response = hyper::Response<screw_core::body::ResponseBody>,
        Error = std::convert::Infallible,
    > + Send,
) {
    let builder = hyper::server::conn::http1::Builder::new();
    if let Err(err) = builder
        .serve_connection(hyper_util::rt::TokioIo::new(stream), session_service)
        .await
    {
        eprintln!("Error serving connection: {err}");
    }
}

pub async fn init_config() -> config::Config {
    config::Config::builder()
        .add_source(config::File::with_name("config"))
        .build()
        .expect("CONFIG init failed")
}

pub async fn init_db() -> rbatis::RBatis {
    let rb = rbatis::RBatis::new();
    rb.init(rbdc_pg::driver::PgDriver {}, &*PG_URL)
        .expect("DB init failed");
    migrations::exec(&rb).await.expect("DB migration failed");
    rb
}
