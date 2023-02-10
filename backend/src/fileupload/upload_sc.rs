use super::post_model::post;
use actix_web::{web, HttpResponse, Scope};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum PostType {
    Global,
    OnlyMe,
    Friends,
}

#[derive(Serialize, Deserialize)]
pub struct Model {
    user_id: String,
    post_type: PostType,
    text: Option<String>,
    images: Option<Vec<String>>,
    videos: Option<Vec<String>>,
}

pub fn post_scope() -> Scope {
    web::scope("/post").route("{posty}", web::post().to(post_handle))
}

pub async fn post_handle(body: web::Json<Model>, posty: web::Path<String>) -> HttpResponse {
    match posty.as_str() {
        "post-post" => post(body).await,
        _ => HttpResponse::BadRequest().await.unwrap(),
    }
}
