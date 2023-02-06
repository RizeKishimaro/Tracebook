use actix_cors::Cors;
use actix_web::{App, HttpServer};
mod scopes;
use dotenvy::dotenv;
use scopes::user::user_scope;
mod extractors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect("Error due to: .env File not found");
    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .app_data(dotenvy::var("SECRET").unwrap())
            .service(user_scope())
    })
    .bind(("127.0.0.1", 8090))?
    .run()
    .await
}
