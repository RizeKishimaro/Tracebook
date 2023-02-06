use actix_web::{web, HttpResponse};
use jsonwebtoken::{decode, errors::Error, DecodingKey, TokenData, Validation};

use super::user::{Claims, DecodeResponse, Info, Response, DB};

pub async fn decode_token(
    (ds, ses): &DB,
    body: web::Json<Info>,
    secret: web::Data<String>,
) -> HttpResponse {
    let decoded: Result<TokenData<Claims>, Error> = decode(
        &body.token,
        &DecodingKey::from_secret(secret.as_str().as_ref()),
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    );

    let sql = "SELECT * FROM user;";
    let resl = ds.execute(sql, ses, None, false).await.unwrap();
    println!("{resl:?}");

    match decoded {
        Ok(token) => HttpResponse::Ok().json(DecodeResponse {
            message: "Authed".to_string(),
            id: token.claims.id,
            username: token.claims.username,
            password: token.claims.password,
        }),
        Err(e) => HttpResponse::BadRequest().json(Response {
            message: e.to_string(),
        }),
    }
}
