use std::collections::BTreeMap;

use actix_web::{web, HttpResponse};
use jsonwebtoken::{decode, errors::Error, DecodingKey, TokenData, Validation};
use rand::random;
use surrealdb::sql::Value;

use crate::scopes::user::Claims;

use super::upload_sc::Model;

pub async fn post(model: web::Json<Model>, secret: web::Data<String>) -> HttpResponse {
    let post_id = random::<u64>();
    let user_info: Result<TokenData<Claims>, Error> = decode(
        &model.user_token,
        &DecodingKey::from_secret(secret.as_str().as_ref()),
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    );
    let sql = format!("CREATE post:{post_id} CONTENT $data;");

    let data: BTreeMap<String, Value> = [
        ("post_type".into(), post_id.into()),
        ("text".into(), model.text.clone().into()),
        ("images".into(), model.images),
    ]
    .into();
    HttpResponse::Ok().await.unwrap()
}
