use std::collections::BTreeMap;

use crate::scopes::user::{Claims, Emnum, EncodeResponse, Info, DB};
use actix_web::{web, HttpResponse};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use rand::random;
use surrealdb::sql::Value;

pub async fn sign_up(
    (ds, ses): &DB,
    body: web::Json<Info>,
    secret: web::Data<String>,
) -> HttpResponse {
    let body = body.user.as_ref().unwrap();
    let id = format!("{}{}", random::<u32>(), body.username.clone());
    let exp = (Utc::now() + Duration::days(365)).timestamp() as usize;

    let sql = format!("CREATE user:{id} CONTENT $data");

    let emnum = match body.emnum.clone() {
        Emnum::Mail(mail) => mail,
        Emnum::Num(num) => num.to_string(),
    };

    let data: BTreeMap<String, Value> = [
        ("user_id".into(), id.clone().into()),
        ("emnum".into(), emnum.into()),
        ("username".into(), body.username.clone().into()),
        ("password".into(), body.password.clone().into()),
        ("sex".into(), format!("{:?}", body.sex).into()),
    ]
    .into();

    let var: BTreeMap<String, Value> = [("data".into(), data.into())].into();

    let claim: Claims = Claims {
        id,
        exp,
        emnum: body.emnum.clone(),
        sex: body.sex.clone(),
        username: body.username.clone(),
        password: body.password.clone(),
    };

    let token: String = encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(secret.as_str().as_ref()),
    )
    .unwrap();

    ds.execute(&sql, ses, Some(var), false).await.unwrap();

    HttpResponse::Ok().json(EncodeResponse {
        message: String::from("success"),
        token,
    })
}
