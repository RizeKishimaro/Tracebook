use actix_web::{web, App, HttpServer};
use dotenvy::var;
mod auth;
mod extra;
mod scopes;
mod structures;
use scopes::auth::auth_scope;

#[actix_web::main]
async fn main() {
    let (host, port) = (var("HOST").unwrap(), var("PORT").unwrap().parse().unwrap());
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new("SECRET"))
            .service(auth_scope())
    })
    .bind((host, port))
    .unwrap()
    .run()
    .await
    .unwrap();
}
