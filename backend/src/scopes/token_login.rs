use super::{
    into_obj::{get_value, obj_str},
    user::{Claims, DecodeResponse, Info, Response, DB},
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
            let sql = format!("SELECT * FROM user:{}", data.id);

            let resul = ds.execute(&sql, ses, None, false).await.unwrap();

            let id_value = get_value(resul).unwrap();
            let vec_field = vec![
                "user_id".to_string(),
                "emnum".to_string(),
                "username".to_string(),
                "password".to_string(),
                "sex".to_string(),
            ];
            let vec_data = obj_str(id_value, vec_field);
            let datas = vec![
                format!("\"{}\"", data.id),
                format!("\"{}\"", data.emnum),
                format!("\"{}\"", data.username),
                format!("\"{}\"", data.password),
                format!("\"{}\"", data.sex),
            ];

            if vec_data == datas {
                HttpResponse::Ok().json(DecodeResponse {
                    message: "Authed".to_string(),
                    id: data.id,
                    token: body.token.clone(),
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
