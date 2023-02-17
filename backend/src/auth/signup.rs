use std::collections::BTreeMap;

use crate::structures::auth_struct::{Claims, EncodeResponse, Info, DB};
use actix_web::{web, HttpResponse};
use argon2::{hash_encoded, verify_encoded_ext, Config};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use rand::random;
use surrealdb::sql::Value;

pub async fn sign_up(
    (ds, ses): &DB,
    body: web::Json<Info>,
    secret: web::Data<String>,
    argon_data: web::Data<Vec<String>>,
) -> HttpResponse {
    let body = body.user.as_ref().unwrap();
    let id = format!("{}{}", random::<u32>(), body.username.clone());
    let exp = (Utc::now() + Duration::days(365)).timestamp() as usize;

    let config = Config {
        ad: argon_data[1].as_bytes(),
        hash_length: 256,
        lanes: 35,
        mem_cost: 99999,
        secret: argon_data[0].as_bytes(),
        thread_mode: argon2::ThreadMode::Parallel,
        time_cost: 5,
        variant: argon2::Variant::Argon2i,
        version: argon2::Version::Version13,
    };

    let hashed_pass = hash_encoded(
        body.password.clone().as_bytes(),
        argon_data[2].as_bytes(),
        &config,
    )
    .unwrap();
    let deco = verify_encoded_ext(
        &hashed_pass,
        body.password.clone().as_bytes(),
        argon_data[0].as_bytes(),
        argon_data[1].as_bytes(),
    )
    .unwrap();

    println!("{}\n\n{}", hashed_pass, deco);

    let sql = format!("CREATE user:{id} CONTENT $data");

    let data: BTreeMap<String, Value> = [
        ("user_id".into(), id.clone().into()),
        ("emnum".into(), body.emnum.clone().into()),
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
