use super::{
    into_obj::into_obj,
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
            let res_value = into_obj(resul)
                .unwrap()
                .next()
                .transpose()
                .unwrap()
                .and_then(|obj| obj.get("user_id").map(|id| id.to_string()));

            println!("{}", res_value.unwrap());

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
