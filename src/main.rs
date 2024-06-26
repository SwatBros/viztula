use actix_web::{App, HttpServer};

mod chart;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(chart::service::chart))
        .bind(("127.0.0.1", 8999))?
        .run()
        .await
}
