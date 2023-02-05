use crate::scopes::user::{Claims, EncodeResponse, Info, DB};
use actix_web::{web, HttpResponse};
use jsonwebtoken::{encode, EncodingKey, Header};
use rand::random;

pub async fn encode_token(
    (ds, ses): &DB,
    body: web::Json<Info>,
    secret: web::Data<String>,
) -> HttpResponse {
    let id = format!("{}{}", random::<u32>(), body.username.clone());
    let sql = format!(
        "CREATE user:{} SET username = '{}', password = '{}';",
        id,
        body.username.clone(),
        body.password.clone()
    );

    let claim: Claims = Claims {
        id,
        username: body.username.clone(),
        password: body.password.clone(),
    };

    let token: String = encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(secret.as_str().as_ref()),
    )
    .unwrap();

    let resul = ds.execute(&sql, ses, None, false).await.unwrap();
    println!("{resul:?}");

    HttpResponse::Ok().json(EncodeResponse {
        message: String::from("success"),
        token,
    })
}
