use std::{fs::File, io::Read, path::Path};

use actix_web::{get, web, App, HttpResponse, HttpServer};
use auth::ch_name::ch_name_fnc;
use dotenvy::var;
use post::fetch_post::fetch_post;
use serde::{Deserialize, Serialize};
use tokio::fs;
mod auth;
mod extra;
mod post;
mod scopes;
mod structures;
use scopes::{auth::auth_scope, post::post_scope};

#[actix_web::main]
async fn main() {
    if !Path::new("./blablauplo").exists() {
        fs::create_dir("./blablauplo").await.unwrap();
    }

    let (host, port) = (var("HOST").unwrap(), var("PORT").unwrap().parse().unwrap());
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(var("SECRET").unwrap()))
            .route("/ch_name", web::route().to(ch_name_fnc))
            .route("/fetch_post", web::route().to(fetch_post))
            .service(get_img)
            .service(auth_scope())
            .service(post_scope())
    })
    .bind((host, port))
    .unwrap()
    .run()
    .await
    .unwrap();
}

#[derive(Serialize, Deserialize)]
struct ByteResp {
    value: Vec<u8>,
}

#[get("/images/{name}")]
async fn get_img(name: web::Path<String>) -> HttpResponse {
    let mut bytes = Vec::new();
    File::open(format!("./user_uploaded_assets/{}", name.as_str()))
        .unwrap()
        .read_to_end(&mut bytes);
    HttpResponse::Ok().json(ByteResp { value: bytes })
}
