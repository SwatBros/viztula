use actix_web::{error::ErrorInternalServerError, get, post, web, HttpResponse, Responder, Result};

use crate::chart::{model::{Chart, Query}, repository};

#[post("/chart")]
pub async fn chart(body: web::Json<Chart>) -> Result<impl Responder> {
    match repository::insert(body.0).await {
        Ok(id) => Ok(HttpResponse::Ok().json(id)),
        Err(e) => Err(ErrorInternalServerError(e)),
    }
}

#[get("/chart/data")]
pub async fn data(body: web::Json<Chart>) -> Result<impl Responder> {
	match repository::query(body.0.query()).await {
        Ok(rows) => Ok(HttpResponse::Ok().json(rows)),
        Err(e) => Err(ErrorInternalServerError(e)),
    }
}