use actix_web::{web, HttpResponse, Scope};
use serde::{Deserialize, Serialize};
use surrealdb::{Datastore, Session};

use super::{normal_login::login, signup::sign_up, token_login::token_login};

pub type DB = (Datastore, Session);

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub id: String,
    pub exp: usize,
    pub emnum: String,
    pub sex: String,
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
    pub emnum: String,
    pub username: String,
    pub password: String,
    pub sex: String,
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
    pub token: String,
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
    } else if method.as_str() == "token-login" {
        token_login(db, body, secret).await
    } else if method.as_str() == "login" {
        login(db, body).await
    } else {
        HttpResponse::BadRequest().await.unwrap()
    }
}
