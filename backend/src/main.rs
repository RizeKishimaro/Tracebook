use actix_web::{web, App, HttpServer};
mod scopes;
use scopes::user::user_scope;
mod extractors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(String::from(
                "abcdefghijklmnopqrstuvwxyzwalkerize0123456789walkerizeABCDEFGHIJKLMNOPQRSTUVWXYZ",
            )))
            .service(user_scope())
    })
    .bind(("127.0.0.1", 8090))?
    .run()
    .await
}
