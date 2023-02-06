use actix_web::{web, HttpResponse, Scope};
use serde::{Deserialize, Serialize};
use surrealdb::{Datastore, Session};

use super::{decode::log_in, encode::sign_up};

pub type DB = (Datastore, Session);

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub id: String,
    pub exp: usize,
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
    pub token: String,
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
    body: web::Json<Info>,
    secret: web::Data<String>,
) -> HttpResponse {
    let db = &(
        Datastore::new("file://tracebook.db").await.unwrap(),
        Session::for_db("trace", "book"),
    );
    if method.as_str() == "signup" {
        sign_up(db, body, secret).await
    } else if method.as_str() == "login" {
        log_in(db, body, secret).await
    } else {
        HttpResponse::BadRequest().await.unwrap()
    }
}
