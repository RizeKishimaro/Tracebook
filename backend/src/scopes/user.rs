use crate::scopes::encode::encode_token;
use actix_web::{web, HttpResponse, Scope};
use serde::{Deserialize, Serialize};
use surrealdb::{Datastore, Session};

use super::decode::decode_token;

pub type DB = (Datastore, Session);

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

#[derive(Serialize, Deserialize)]
pub struct DecodeBody {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct DecodeResponse {
    pub message: String,
    pub id: String,
    pub username: String,
    pub password: String,
}

pub fn user_scope() -> Scope {
    web::scope("/user").route("{method}", web::post().to(branch))
}

pub async fn branch(
    method: web::Path<String>,
    debody: web::Json<DecodeBody>,
    body: web::Json<Info>,
    secret: web::Data<String>,
) -> HttpResponse {
    let db = &(
        Datastore::new("memory").await.unwrap(),
        Session::for_db("trace", "book"),
    );
    if method.as_str() == "encode-token" {
        encode_token(db, body, secret).await
    } else if method.as_str() == "decode-token" {
        decode_token(debody, secret).await
    } else {
        HttpResponse::BadRequest().await.unwrap()
    }
}
