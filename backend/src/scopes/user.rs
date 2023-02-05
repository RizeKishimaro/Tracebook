use crate::extractors::auth_token::AuthToken;
use crate::scopes::encode::encode_token;
use actix_web::{web, Scope};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub id: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct EncodeResponse {
    pub message: String,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct Info {
    pub username: String,
    pub password: String,
}

pub fn user_scope() -> Scope {
    web::scope("/user").route("encode-token", web::post().to(encode_token))
}
