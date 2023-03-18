use std::path::Path;

use actix_web::{web, App, HttpServer};
use auth::ch_name::ch_name_fnc;
use dotenvy::var;
use tokio::fs;
mod auth;
mod extra;
mod post;
mod scopes;
mod structures;
use post::upload_post::post_upload;
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
            .route("/post", web::route().to(post_upload))
            .service(auth_scope())
            .service(post_scope())
    })
    .bind((host, port))
    .unwrap()
    .run()
    .await
    .unwrap();
}
