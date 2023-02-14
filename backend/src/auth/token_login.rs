use crate::{
    extra::into_obj::get_value,
    scopes::user::{Claims, DecodeResponse, Info, Response, DB},
};
use actix_web::{web, HttpResponse};
use jsonwebtoken::{decode, errors::Error, DecodingKey, TokenData, Validation};

use super::check_user::check_user;

pub async fn token_login(
    (ds, ses): &DB,
    body: web::Json<Info>,
    secret: web::Data<String>,
) -> HttpResponse {
    let token_stru = body.token.clone().unwrap();
    let decoded: Result<TokenData<Claims>, Error> = decode(
        &token_stru,
        &DecodingKey::from_secret(secret.as_str().as_ref()),
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    );

    match decoded {
        Ok(token) => {
            let data = token.claims;

            let sql = format!("SELECT * FROM user:{};", data.id);

            let resul = ds.execute(&sql, ses, None, false).await.unwrap();

            let check = check_user(data.clone(), resul);

            match check {
                Ok(_) => HttpResponse::Ok().json(DecodeResponse {
                    message: "Authed".to_string(),
                    id: data.id,
                    token: token_stru,
                }),
                Err(e) => HttpResponse::BadRequest().json(Response { message: e }),
            }
        }
        Err(e) => HttpResponse::BadRequest().json(Response {
            message: e.to_string(),
        }),
    }
}
