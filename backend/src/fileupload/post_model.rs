use actix_web::{web, HttpResponse};

use super::upload_sc::Model;

pub async fn post(model: web::Json<Model>) -> HttpResponse {
    HttpResponse::Ok().await.unwrap()
}
