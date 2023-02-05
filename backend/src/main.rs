use std::future::Future;

use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer};
mod scopes;
use scopes::user::user_scope;
use surrealdb::{Datastore, Session};
mod extractors;

struct DB {
    ds: Datastore,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut db: &Future<Output = Datastore> = &gtk();
    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .app_data((Session::for_db("rep", "opb")))
            .app_data(web::Data::new(String::from(
                "abcdefghijklmnopqrstuvwxyzwalkerize0123456789walkerizeABCDEFGHIJKLMNOPQRSTUVWXYZ",
            )))
            .service(user_scope())
    })
    .bind(("127.0.0.1", 8090))?
    .run()
    .await
}

async fn gtk() -> Datastore {
    Datastore::new("memory").await.unwrap()
}
