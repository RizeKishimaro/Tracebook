use actix_web::{web, HttpResponse};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use rand::random;

use crate::{
    extra::into_obj::get_value,
    scopes::user::{Claims, EncodeResponse, Info, Response, DB},
};

pub async fn login(
    (ds, ses): &DB,
    body: web::Json<Info>,
    secret: web::Data<String>,
) -> HttpResponse {
    let sql = format!(
        "SELECT * FROM user WHERE emnum = \"{}\" AND username = \"{}\" AND password = \"{}\" AND sex = \"{}\";",
        body.emnum.clone(),
        body.username.clone(),
        body.password.clone(),
        body.sex.clone()
    );

    let resul = ds.execute(&sql, ses, None, true).await.unwrap();

    let check = get_value(resul);

    match check {
        Ok(_) => {
            let id = format!("{}{}", random::<u32>(), body.username.clone());
            let exp = (Utc::now() + Duration::days(365)).timestamp() as usize;

            let claims: Claims = Claims {
                id,
                exp,
                emnum: body.emnum.clone(),
                sex: body.sex.clone(),
                username: body.username.clone(),
                password: body.password.clone(),
            };

            let token: String = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(secret.as_str().as_ref()),
            )
            .unwrap();

            HttpResponse::Ok().json(EncodeResponse {
                message: "success".to_string(),
                token,
            })
        }

        Err(e) => HttpResponse::BadRequest().json(Response { message: e }),
    }
}
