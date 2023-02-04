use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer};
mod scopes;
use scopes::user::user_scope;
mod extractors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(String::from(
                "abcdefghijklmnopqrstuvwxyzwalkerize0123456789walkerizeABCDEFGHIJKLMNOPQRSTUVWXYZ",
            )))
            .service(user_scope())
    })
    .bind(("127.0.0.1", 8090))?
    .run()
    .await
}

