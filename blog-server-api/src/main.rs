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

    let addr = std::env::var("SERVER_ADDRESS")
        .expect("SERVER_ADDRESS expected in env vars")
        .parse()?;
    println!("Listening on http://{}", addr);
    hyper::Server::bind(&addr).serve(server_service).await?;

    Ok(())
}

#[derive(Debug)]
struct DbgRbatisIntercept;

impl rbatis::intercept::Intercept for DbgRbatisIntercept {
    fn before(
        &self,
        _task_id: i64,
        _rb: &dyn rbatis::executor::Executor,
        sql: &mut String,
        args: &mut Vec<rbs::Value>,
    ) -> Result<(), rbatis::Error> {
        dbg!(sql);
        dbg!(args);
        Ok(())
    }
}

pub async fn init_db() -> rbatis::RBatis {
    let opt = rbatis::RBatisOption {
        intercepts: {
            let intercepts: rbatis::dark_std::sync::SyncVec<
                std::sync::Arc<dyn rbatis::intercept::Intercept>,
            > = rbatis::dark_std::sync::SyncVec::new();
            if cfg!(debug_assertions) {
                intercepts.push(std::sync::Arc::new(DbgRbatisIntercept));
            }
            intercepts
        },
    };
    let rb = rbatis::RBatis::new_with_opt(opt);
    rb.init(
        rbdc_pg::driver::PgDriver {},
        std::env::var("PG_URL")
            .expect("PG_URL expected in env vars")
            .as_str(),
    )
    .unwrap();

    let sql = std::fs::read_to_string("./table_pg.sql").unwrap();
    rb.exec(&sql, vec![]).await.expect("DB migration failed");
    return rb;
}
