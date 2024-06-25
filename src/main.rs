use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use tokio_postgres::NoTls;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("127.0.0.1", 8999))?
        .run()
        .await
}

#[get("/")]
async fn hello() -> impl Responder {
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres password=postgres", NoTls)
            .await
            .unwrap();

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Now we can execute a simple statement that just returns its parameter.
    let rows = client.query("SELECT $1::TEXT", &[&"hello world"]).await;

    if let Ok(rows) = rows {
        let value: String = rows[0].get(0);
        HttpResponse::Ok().body(value)
    } else {
        HttpResponse::InternalServerError().finish()
    }
}
