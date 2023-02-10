use super::post_model::post;
use actix_web::{web, HttpResponse, Scope};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Model {
    text: String,
    images: Vec<String>,
    videos: Vec<String>,
}

pub fn post_scope() -> Scope {
    web::scope("/post").route("{posty}", web::post().to(post_handle))
}

pub async fn post_handle(body: web::Json<Model>, posty: web::Path<String>) -> HttpResponse {
    match posty.as_str() {
        "post-post" => post().await,
        _ => HttpResponse::BadRequest().await.unwrap(),
    }
}
