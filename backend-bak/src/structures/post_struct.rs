use super::post_enum::*;
use serde::{Deserialize, Serialize};

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
