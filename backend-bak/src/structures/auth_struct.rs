use super::auth_enum::Sex;
use serde::{Deserialize, Serialize};
use surrealdb::{Datastore, Session};

pub type DB = (Datastore, Session);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Claims {
    pub id: String,
    pub exp: usize,
    pub emnum: String,
    pub sex: Sex,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct EncodeResponse {
    pub message: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo {
    pub emnum: String,
    pub username: String,
    pub password: String,
    pub sex: Sex,
}

#[derive(Serialize, Deserialize)]
pub struct Info {
    pub token: Option<String>,
    pub user: Option<UserInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct DecodeBody {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct DecodeResponse {
    pub message: String,
    pub id: String,
    pub token: String,
}
