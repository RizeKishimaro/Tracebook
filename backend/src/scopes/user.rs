use crate::scopes::encode::encode_token;
use actix_web::{web, HttpResponse, Scope};
use serde::{Deserialize, Serialize};
use surrealdb::{Datastore, Session};

type DB = (Datastore, Session);

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
    web::scope("/user").route("{method}", web::post().to(branch))
}

pub async fn branch(
    method: web::Path<String>,
    body: web::Json<Info>,
    secret: web::Data<String>,
) -> HttpResponse {
    let db = &(
        Datastore::new("memory").await.unwrap(),
        Session::for_db("trace", "book"),
    );
    let (ds, ses) = db;
    if method.as_str() == "encode-token" {
        encode_token(body, secret).await
    } else {
        HttpResponse::BadRequest().await.unwrap()
    }
}
