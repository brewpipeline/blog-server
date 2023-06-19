mod endpoints;
mod entities;
mod extensions;
mod router;
mod utils;

const SERVER_ADDRESS: &str = "127.0.0.1:3000";

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
    let rb = rbatis::RBatis::new();
    rb.init(
        rbdc_sqlite::driver::SqliteDriver {},
        "sqlite://target/blog.db",
    )
    .unwrap();

    // let sql = std::fs::read_to_string("./table_mysql.sql").unwrap();
    // let _ = rb.exec(&sql, vec![]).await;
    return rb;
}
