use actix_web::{web, App, HttpResponse, HttpServer};
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
            .route("/", web::get().to(root))
            .service(user_scope())
    })
    .bind(("127.0.0.1", 8090))?
    .run()
    .await
}

async fn root() -> HttpResponse {
    HttpResponse::Ok().body(include_str!("./login/login.html"))
}
