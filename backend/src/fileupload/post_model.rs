use std::collections::BTreeMap;

use actix_web::{web, HttpResponse};
use jsonwebtoken::{decode, errors::Error, DecodingKey, TokenData, Validation};
use rand::random;
use surrealdb::sql::Value;

use crate::scopes::user::{Claims, DB};

use super::upload_sc::Model;

pub async fn post(
    (ds, ses): &DB,
    model: web::Json<Model>,
    secret: web::Data<String>,
) -> HttpResponse {
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
        ("images".into(), format!("{:?}", model.images).into()),
        ("videos".into(), format!("{:?}", model.videos).into()),
        (
            "user_poster".into(),
            format!("user:{}", user_info.unwrap().claims.id).into(),
        ),
    ]
    .into();

    let var: BTreeMap<String, Value> = [("data".into(), data.into())].into();
    let _post = ds.execute(&sql, ses, Some(var), false).await.unwrap();
    HttpResponse::Ok().await.unwrap()
}
