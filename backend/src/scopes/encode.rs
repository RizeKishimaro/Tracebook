use crate::scopes::user::{Claims, EncodeResponse, Info, DB};
use actix_web::{web, HttpResponse};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use rand::random;

pub async fn encode_token(
    (ds, ses): &DB,
    body: web::Json<Info>,
    secret: web::Data<String>,
) -> HttpResponse {
    let id = format!("{}{}", random::<u32>(), body.username.clone());
    let exp = (Utc::now() + Duration::days(365)).timestamp() as usize;
    let sql = format!(
        "CREATE user:{} SET username = '{}', password = '{}';",
        id,
        body.username.clone(),
        body.password.clone()
    );

    let claim: Claims = Claims {
        id,
        exp,
        username: body.username.clone(),
        password: body.password.clone(),
    };

    let token: String = encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(secret.as_str().as_ref()),
    )
    .unwrap();

    ds.execute(&sql, ses, None, false).await.unwrap();

    HttpResponse::Ok().json(EncodeResponse {
        message: String::from("success"),
        token,
    })
}
