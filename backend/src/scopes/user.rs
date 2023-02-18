use actix_web::{web, HttpResponse, Scope};
use argon2::Config;
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
    let body_idk = body.user.as_ref().unwrap();
    let extra_sec = format!(
        "{}{}{}",
        body_idk.username.clone(),
        body_idk.password.clone(),
        body_idk.emnum.clone()
    );
    let (argon_sec, argon_ad) = (
        format!("{}{}", argon_data[0].clone(), extra_sec),
        format!("{}{}", argon_data[1].clone(), extra_sec),
    );
    let config = Config {
        ad: argon_ad.as_bytes(),
        hash_length: 256,
        lanes: 35,
        mem_cost: 99999,
        secret: argon_sec.as_bytes(),
        thread_mode: argon2::ThreadMode::Parallel,
        time_cost: 3,
        variant: argon2::Variant::Argon2i,
        version: argon2::Version::Version13,
    };

    match method.as_str() {
        "login" => login(db, body, secret, argon_data.clone(), config, extra_sec).await,
        "signup" => sign_up(db, body, secret, argon_data.clone(), extra_sec, config).await,
        "token-login" => token_login(db, body, secret).await,
        _ => HttpResponse::BadRequest().json(Response {
            message: "idk".to_string(),
        }),
    }
}
