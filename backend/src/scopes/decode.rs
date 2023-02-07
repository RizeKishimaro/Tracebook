use super::{
    into_obj::{get_value, obj_str},
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
            let data = token.claims;
            let sql = format!("SELECT * FROM user:{}", data.id);

            let resul = ds.execute(&sql, ses, None, false).await.unwrap();

            let id_value = get_value(resul).unwrap();
            let obj_str = obj_str(id_value, "user_id".to_string()).unwrap();
            let ch_id = format!("\"{}\"", data.id);

            if obj_str == ch_id {
                HttpResponse::Ok().json(DecodeResponse {
                    message: "Authed".to_string(),
                    id: data.id,
                    username: data.username,
                    password: data.password,
                })
            } else {
                HttpResponse::Unauthorized().json(Response {
                    message: "Unauthorized!".to_string(),
                })
            }
        }
        Err(e) => HttpResponse::BadRequest().json(Response {
            message: e.to_string(),
        }),
    }
}
