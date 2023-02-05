use crate::scopes::user::{Claims, Info};
use actix_web::{web, HttpResponse};
use rand::random;

pub async fn encode_token(body: web::Json<Info>, secret: web::Data<String>) -> HttpResponse {
    let id = format!("{}{}", random::<u32>(), body.username);

    HttpResponse::Ok().await.unwrap()
}
