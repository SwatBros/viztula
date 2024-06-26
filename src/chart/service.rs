use actix_web::{error::ErrorInternalServerError, post, web, HttpResponse, Responder, Result};

use crate::chart::{model::Chart, repository};

#[post("/chart")]
pub async fn chart(body: web::Json<Chart>) -> Result<impl Responder> {
    match repository::insert(body.0).await {
        Ok(id) => Ok(HttpResponse::Ok().json(id)),
        Err(e) => Err(ErrorInternalServerError(e)),
    }
}
