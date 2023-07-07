#![allow(dead_code)]

mod endpoints;
mod entities;
mod extensions;
mod router;
mod utils;

#[tokio::main]
async fn main() -> screw_components::dyn_result::DResult<()> {
    dotenv::dotenv().ok();

    let rbatis = init_db().await;

    let server_service = screw_core::server::ServerService::with_responder_factory(
        screw_core::responder_factory::ResponderFactory::with_router(router::make_router())
            .and_extensions(extensions::make_extensions(rbatis)),
    );

    let addr = std::env::var("SERVER_ADDRESS").expect("SERVER_ADDRESS expected in env vars").parse()?;
    println!("Listening on http://{}", addr);
    hyper::Server::bind(&addr).serve(server_service).await?;

    Ok(())
}

pub async fn init_db() -> rbatis::RBatis {
    let rb = rbatis::RBatis::new();
    rb.init(
        rbdc_mysql::driver::MysqlDriver {},
        std::env::var("MYSQL_URL").expect("MYSQL_URL expected in env vars").as_str(),
    )
    .unwrap();

    let sql = std::fs::read_to_string("./table_mysql.sql").unwrap();
    let _ = rb.exec(&sql, vec![]).await;
    return rb;
}
