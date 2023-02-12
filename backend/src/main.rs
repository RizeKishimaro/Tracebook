use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use fileupload::upload_sc::post_scope;
use scopes::user::user_scope;
use surrealdb::{Datastore, Session};
mod auth;
mod extra;
mod extractors;
mod fileupload;
mod scopes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect("Error due to: .env File not found");
    HttpServer::new(|| {
        let cors = Cors::permissive();
        let secret = dotenvy::var("SECRET").unwrap();
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(secret))
            .service(post_scope())
            .service(user_scope())
    })
    .bind(("127.0.0.1", 8090))?
    .run()
    .await
}
