use crate::{
    extra::into_obj::get_value,
    scopes::user::{Claims, DecodeResponse, Emnum, Info, Response, DB},
};
use actix_web::{web, HttpResponse};
use jsonwebtoken::{decode, errors::Error, DecodingKey, TokenData, Validation};

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

            let emnum = match data.emnum {
                Emnum::Num(num) => num.to_string(),
                Emnum::Mail(mail) => mail,
            };

            let sql = format!("SELECT * FROM user:{} WHERE emnum = \"{}\" AND username = \"{}\" AND password = \"{}\" AND sex = \"{:?}\";", data.id, emnum, data.username, data.password, data.sex);

            let resul = ds.execute(&sql, ses, None, false).await.unwrap();

            let check = get_value(resul);

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
