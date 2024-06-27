use actix_web::{error::ErrorInternalServerError, get, post, web, HttpResponse, Responder, Result};
use sqlx::PgPool;

use crate::chart::{
    model::{Chart, Query},
    repository,
};

#[post("/chart")]
pub async fn chart(pool: web::Data<PgPool>, body: web::Json<Chart>) -> Result<impl Responder> {
    match repository::insert(&pool, body.0).await {
        Ok(id) => Ok(HttpResponse::Ok().json(id)),
        Err(e) => Err(ErrorInternalServerError(e)),
    }
}

#[get("/chart/data")]
pub async fn data(body: web::Json<Chart>) -> Result<impl Responder> {
    let pool = PgPool::connect(
        format!(
            "postgresql://postgres:password@localhost:5432/{}",
            body.0.db
        )
        .as_str(),
    )
    .await
    .unwrap();

    match repository::query(&pool, body.0.query()).await {
        Ok(rows) => Ok(HttpResponse::Ok().json(rows)),
        Err(e) => Err(ErrorInternalServerError(e)),
    }
}
