use actix_web::{web, HttpResponse, Scope};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

pub fn user_scope() -> Scope {
    web::scope("/user")
        .route("/encode-token/{id}", web::get().to(encode_token))
        .route("decode-token", web::post().to(decode_token))
        .route("protected", web::get().to(protected))
}

#[derive(Serialize, Deserialize)]
struct Claims {
    id: usize,
    exp: usize,
}

#[derive(Serialize, Deserialize)]
struct Response {
    message: String,
}

#[derive(Serialize, Deserialize)]
struct EncodeResponse {
    message: String,
    token: String,
}

async fn encode_token(path: web::Path<usize>, secret: web::Data<String>) -> HttpResponse {
    let id: usize = path.into_inner();
    let exp: usize = (Utc::now() + Duration::days(365)).timestamp() as usize;
    let claim: Claims = Claims { id, exp };
    let token: String = encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(secret.as_str().as_ref()),
    )
    .unwrap();
    HttpResponse::Ok().json(EncodeResponse {
        message: String::from("success"),
        token,
    })
}

#[derive(Serialize, Deserialize)]
struct DecodeBody {
    token: String,
}

async fn decode_token(body: web::Json<DecodeBody>, secret: web::Data<String>) -> HttpResponse {
    HttpResponse::Ok().json(Response {
        message: String::from("decode-token"),
    })
}

async fn protected() -> HttpResponse {
    HttpResponse::Ok().json(Response {
        message: String::from("protected"),
    })
}
