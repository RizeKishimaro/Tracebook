use actix_web::{web, HttpResponse, Scope};
use argon2::{hash_encoded, Config};
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
    argon_data: web::Data<Vec<String>>,
) -> HttpResponse {
    let db = &(
        Datastore::new("file://tracebook.db").await.unwrap(),
        Session::for_db("trace", "book"),
    );
    let config = Config {
        ad: argon_data[1].as_bytes(),
        hash_length: 256,
        lanes: 35,
        mem_cost: 99999,
        secret: argon_data[0].as_bytes(),
        thread_mode: argon2::ThreadMode::Parallel,
        time_cost: 3,
        variant: argon2::Variant::Argon2i,
        version: argon2::Version::Version13,
    };

    match method.as_str() {
        "login" => login(db, body, secret).await,
        "signup" => sign_up(db, body, secret, argon_data.clone(), config).await,
        "token-login" => token_login(db, body, secret).await,
        _ => HttpResponse::BadRequest().json(Response {
            message: "idk".to_string(),
        }),
    }
}
