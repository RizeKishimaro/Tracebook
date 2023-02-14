use super::post_model::post;
use actix_web::{web, HttpResponse, Scope};
use serde::*;
use serde_json::{self, from_str};
use surrealdb::{Datastore, Session};

type DB = (Datastore, Session);

#[derive(Serialize, Deserialize, Debug)]
pub enum PostType {
    Global,
    OnlyMe,
    Friends,
}

impl From<String> for PostType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Global" => PostType::Global,
            "OnlyMe" => PostType::OnlyMe,
            "Friends" => PostType::Friends,
            _ => PostType::Global,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Links {
    Links(Vec<String>),
    None(String),
}

impl From<String> for Links {
    fn from(value: String) -> Self {
        let value = &value[1..value.len() - 1];
        match value {
            "None" => Links::None("None".to_string()),
            _ => Links::Links(from_str(value).unwrap()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    pub user_token: String,
    pub post_type: PostType,
    pub text: Option<String>,
    pub images: Links,
    pub videos: Links,
}

#[derive(Serialize, Deserialize)]
pub struct ResponsePost {
    pub post_id: u32,
    pub post_type: PostType,
    pub text: Option<String>,
    pub images: Links,
    pub videos: Links,
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
