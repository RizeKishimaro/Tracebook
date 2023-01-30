use actix_web::{App, HttpServer};
mod scopes;
use scopes::user::user_scope;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(user_scope()))
        .bind(("127.0.0.1", 8090))?
        .run()
        .await
}
