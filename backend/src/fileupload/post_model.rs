use actix_web::{web, HttpResponse};
use rand::random;

use super::upload_sc::Model;

pub async fn post(model: web::Json<Model>) -> HttpResponse {
    let post_id = random::<u128>();
    HttpResponse::Ok().await.unwrap()
}
