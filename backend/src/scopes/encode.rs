use crate::scopes::user;
use actix_web::HttpResponse;

pub async fn encode_token() -> HttpResponse {
    HttpResponse::Ok().await.unwrap()
}
