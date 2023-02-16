use actix_web::{web, HttpResponse, Scope};
use surrealdb::{Datastore, Session};

use crate::{
    auth::{normal_login::login, signup::sign_up, token_login::token_login},
    structures::auth_struct::*,
};

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

    match method.as_str() {
        "login" => login(db, body, secret).await,
        "signup" => sign_up(db, body, secret).await,
        "token-login" => token_login(db, body, secret).await,
        _ => HttpResponse::BadRequest().json(Response {
            message: "idk".to_string(),
        }),
    }
}
