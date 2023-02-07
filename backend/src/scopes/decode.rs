use super::{
    into_obj::get_value,
    user::{Claims, DecodeResponse, Info, Response, DB},
};
use actix_web::{web, HttpResponse};
use jsonwebtoken::{decode, errors::Error, DecodingKey, TokenData, Validation};

pub async fn log_in(
    (ds, ses): &DB,
    body: web::Json<Info>,
    secret: web::Data<String>,
) -> HttpResponse {
    let decoded: Result<TokenData<Claims>, Error> = decode(
        &body.token,
        &DecodingKey::from_secret(secret.as_str().as_ref()),
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    );

    match decoded {
        Ok(token) => {
            let sql = format!("SELECT * FROM user:{}", token.claims.id.clone());

            let resul = ds.execute(&sql, ses, None, false).await.unwrap();
            println!("{resul:?}");
            let res_value = get_value(resul, "user_id").unwrap();
            println!("{res_value}");

            HttpResponse::Ok().json(DecodeResponse {
                message: "Authed".to_string(),
                id: token.claims.id,
                username: token.claims.username,
                password: token.claims.password,
            })
        }
        Err(e) => HttpResponse::BadRequest().json(Response {
            message: e.to_string(),
        }),
    }
}
