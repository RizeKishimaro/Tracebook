use crate::extractors::auth_token::AuthToken;
use actix_web::{web, HttpResponse, Scope};
use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, errors::Error, Algorithm, DecodingKey, EncodingKey, Header, TokenData,
    Validation,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub fn user_scope() -> Scope {
    web::scope("/user")
        .route("/encode-token", web::post().to(encode_token))
        .route("decode-token", web::post().to(decode_token))
        .route("protected", web::get().to(protected))
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub id: Uuid,
    pub exp: usize,
    pub username: String,
    pub password: String,
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

#[derive(Serialize, Deserialize)]
struct DecodeResponse {
    message: String,
    id: Uuid,
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct Info {
    username: String,
    password: String,
}

async fn encode_token(body: web::Json<Info>, secret: web::Data<String>) -> HttpResponse {
    let id = Uuid::new_v4();
    let exp: usize = (Utc::now() + Duration::days(365)).timestamp() as usize;
    let claim: Claims = Claims {
        id,
        exp,
        username: body.username.clone(),
        password: body.password.clone(),
    };
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
    let decoded: Result<TokenData<Claims>, Error> = decode::<Claims>(
        &body.token,
        &DecodingKey::from_secret(secret.as_str().as_ref()),
        &Validation::new(Algorithm::HS256),
    );
    match decoded {
        Ok(token) => HttpResponse::Ok().json(DecodeResponse {
            message: "Authed".to_string(),
            id: token.claims.id,
            username: token.claims.username,
            password: token.claims.password,
        }),
        Err(e) => HttpResponse::BadRequest().json(Response {
            message: e.to_string(),
        }),
    }
}

async fn protected(aut_token: AuthToken) -> HttpResponse {
    println!("{}", aut_token.id);
    HttpResponse::Ok().json(Response {
        message: String::from("protected"),
    })
}
