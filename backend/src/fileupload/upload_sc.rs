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
pub struct Model<'a> {
    pub user_token: String,
    pub post_type: PostType,
    pub text: Option<String>,
    pub images: Option<Vec<&'a str>>,
    pub videos: Option<Vec<String>>,
}

pub fn post_scope() -> Scope {
    web::scope("/post").route("{posty}", web::post().to(post_handle))
}

pub async fn post_handle(
    body: web::Json<Model>,
    posty: web::Path<String>,
    secret: web::Data<String>,
) -> HttpResponse {
    match posty.as_str() {
        "post-post" => post(body, secret).await,
        _ => HttpResponse::BadRequest().await.unwrap(),
    }
}
