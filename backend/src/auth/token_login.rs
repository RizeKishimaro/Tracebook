use crate::{
    extra::into_obj::get_value,
    scopes::user::{Claims, DecodeResponse, Info, Response, DB},
};
use actix_web::{web, HttpResponse};
use jsonwebtoken::{decode, errors::Error, DecodingKey, TokenData, Validation};

pub async fn token_login(
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
            let data = token.claims;
            let sql = format!("SELECT * FROM user:{} WHERE emnum = \"{}\" AND username = \"{}\" AND password = \"{}\" AND sex = \"{}\";", data.id, data.emnum, data.username, data.password, data.sex);

            let resul = ds.execute(&sql, ses, None, false).await.unwrap();

            let check = get_value(resul);

            match check {
                Ok(_) => HttpResponse::Ok().json(DecodeResponse {
                    message: "Authed".to_string(),
                    id: data.id,
                    token: body.token.clone(),
                }),
                Err(e) => HttpResponse::BadRequest().json(Response { message: e }),
            }
        }
        Err(e) => HttpResponse::BadRequest().json(Response {
            message: e.to_string(),
        }),
    }
}
