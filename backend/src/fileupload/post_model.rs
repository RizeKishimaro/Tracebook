use actix_web::HttpResponse;

pub async fn post() -> HttpResponse {
    HttpResponse::Ok().await.unwrap()
}
