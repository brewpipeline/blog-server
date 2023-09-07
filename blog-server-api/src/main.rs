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

#[tokio::main]
async fn main() -> screw_components::dyn_result::DResult<()> {
    let rbatis = init_db().await;

    let server_service = screw_core::server::ServerService::with_responder_factory(
        screw_core::responder_factory::ResponderFactory::with_router(router::make_router())
            .and_extensions(extensions::make_extensions(rbatis)),
    );

    let addr = SERVER_ADDRESS.parse()?;
    println!("Listening on http://{}", addr);
    hyper::Server::bind(&addr).serve(server_service).await?;

    Ok(())
}

pub async fn init_db() -> rbatis::RBatis {
    let rb = rbatis::RBatis::new_with_opt(rbatis::RBatisOption::default());
    rb.init(rbdc_pg::driver::PgDriver {}, PG_URL).unwrap();

    let sql = std::fs::read_to_string("./table_pg.sql").unwrap();
    rb.exec(&sql, vec![]).await.expect("DB migration failed");
    return rb;
}
