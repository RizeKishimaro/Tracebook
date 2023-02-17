use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dotenvy::var;
use extra::config::vec_vars;
use scopes::{upload_sc::post_scope, user::user_scope};

mod auth;
mod extra;
mod extractors;
mod fileupload;
mod scopes;
mod structures;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive();
        let vars_vec = vec!["SECRET_ARGON", "AD", "SALT"];
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(vec_vars(vars_vec)))
            .app_data(web::Data::new(var("SECRET").unwrap()))
            .service(post_scope())
            .service(user_scope())
    })
    .bind((var("HOST").unwrap(), var("PORT").unwrap().parse().unwrap()))?
    .run()
    .await
}
