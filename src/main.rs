use actix_web::{web::Data, App, HttpServer};
use sqlx::PgPool;

mod chart;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    
    let pool = PgPool::connect("postgresql://postgres:password@localhost:5432/postgres")
        .await
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(chart::service::chart)
            .service(chart::service::data)
    })
    .bind(("127.0.0.1", 8999))?
    .run()
    .await
}
