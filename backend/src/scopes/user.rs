use actix_web::{web, HttpResponse, Scope};

pub fn user_scope() -> Scope {
    web::scope("/user")
        .route("/encode-token/{id}", web::get().to(encode_token))
        .route("decode-token", web::get().to(decode_token))
        .route("protected", web::get().to(protected))
}

async fn encode_token(path: web::Path<usize>, secret: web::Data<String>) -> HttpResponse {
    HttpResponse::Ok().json(Response {
        message: "encode_token".to_owned(),
    })
}
