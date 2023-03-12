use crate::{
    fileupload::post_model::post,
    structures::{auth_struct::DB, post_struct::Model},
};
use actix_web::{web, HttpResponse, Scope};
use surrealdb::{Datastore, Session};

pub fn post_scope() -> Scope {
    web::scope("/post").route("{posty}", web::post().to(post_handle))
}

pub async fn post_handle(
    body: web::Json<Model>,
    posty: web::Path<String>,
    secret: web::Data<String>,
) -> HttpResponse {
    let db: &DB = &(
        Datastore::new("file://tracebook.db").await.unwrap(),
        Session::for_db("trace", "book"),
    );
    match posty.as_str() {
        "postpo" => post(db, body, secret).await,
        _ => HttpResponse::BadRequest().await.unwrap(),
    }
}
