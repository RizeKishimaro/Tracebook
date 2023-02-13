use super::post_model::post;
use actix_web::{web, HttpResponse, Scope};
use serde::*;
use surrealdb::{Datastore, Session};

type DB = (Datastore, Session);

#[derive(Serialize, Deserialize, Debug)]
pub enum PostType {
    Global,
    OnlyMe,
    Friends,
}

impl From for PostType {
    fn from(value: T) -> Self {}
}

#[derive(Serialize, Deserialize)]
pub struct PostResponse {
    pub post_type: PostType,
    pub post_id: i32,
    pub text: String,
    pub images: String,
    pub videos: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Links {
    Links(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    pub user_token: String,
    pub post_type: PostType,
    pub text: Option<String>,
    pub images: Option<Links>,
    pub videos: Option<Links>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponsePost {
    pub post_id: u64,
    pub post_type: PostType,
    pub text: Option<String>,
    pub images: Option<Links>,
    pub videos: Option<Links>,
}

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
