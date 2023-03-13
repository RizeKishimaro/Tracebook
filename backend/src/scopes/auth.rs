use crate::{
    auth::{login::login, signup::signup},
    structures::{ReqInfo, Resp, DB},
};
use actix_web::{web, HttpResponse, Scope};
use surrealdb::{Datastore, Session};

pub fn auth_scope() -> Scope {
    web::scope("/auth").route("{method}", web::post().to(auth_branch))
}

pub async fn auth_branch(
    method: web::Path<String>,
    secret: web::Data<String>,
    info: web::Json<ReqInfo>,
) -> HttpResponse {
    let db: &DB = &(
        Datastore::new("file://tracebook.db").await.unwrap(),
        Session::for_db("trace", "book"),
    );

    match method.as_str() {
        "signup" => signup(db, info, secret).await,
        "login" => login(db, info, secret).await,
        _ => HttpResponse::NotFound().json(Resp {
            message: "Method not Found!".into(),
            value: "Just panic!".into(),
        }),
    }
}
