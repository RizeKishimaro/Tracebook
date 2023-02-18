use crate::{
    extra::into_obj::get_value,
    structures::auth_struct::{Claims, DecodeResponse, Info, Response, DB},
};
use actix_web::{web, HttpResponse};
use jsonwebtoken::{decode, errors::Error, DecodingKey, TokenData, Validation};

pub async fn token_login(
    (ds, ses): &DB,
    body: web::Json<Info>,
    secret: web::Data<String>,
) -> HttpResponse {
    match body.token.clone() {
        Some(token_idk) => {
            let decoded: Result<TokenData<Claims>, Error> = decode(
                &token_idk,
                &DecodingKey::from_secret(secret.as_str().as_ref()),
                &Validation::new(jsonwebtoken::Algorithm::HS256),
            );

            match decoded {
                Ok(token) => {
                    let data = token.claims.clone();

                    let sql = format!(
        "SELECT * FROM user:{} WHERE emnum = \"{}\" AND username = \"{}\" AND password = \"{}\" AND sex = \"{:?}\";",
        data.id,
        data.emnum,
        data.username,
        data.password,
        data.sex
    );
                    let resul = ds.execute(&sql, ses, None, false).await.unwrap();

                    match get_value(resul) {
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
        None => HttpResponse::BadRequest().json(Response {
            message: "WTF None?".to_string(),
        }),
    }
}
