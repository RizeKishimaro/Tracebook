use std::collections::BTreeMap;

use crate::structures::auth_struct::{Claims, EncodeResponse, Info, Response, DB};
use actix_web::{web, HttpResponse};
use argon2::{hash_encoded, Config};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use rand::random;
use surrealdb::sql::Value;

pub async fn sign_up(
    (ds, ses): &DB,
    body: web::Json<Info>,
    secret: web::Data<String>,
    argon_salt: String,
    argon_config: Config<'_>,
) -> HttpResponse {
    let body_resul = body.user.as_ref();
    match body_resul {
        Some(body) => {
            let id = format!("{}{}", random::<u32>(), body.username.clone());
            let exp = (Utc::now() + Duration::days(365)).timestamp() as usize;
            let sql = format!("CREATE user:{id} CONTENT $data");

            let hashed_pass = hash_encoded(
                body.password.clone().as_bytes(),
                argon_salt.as_bytes(),
                &argon_config,
            );

            match hashed_pass {
                Ok(hash_pass) => {
                    let hashed_emnum = hash_encoded(
                        body.emnum.clone().as_bytes(),
                        argon_salt.as_bytes(),
                        &argon_config,
                    );
                    match hashed_emnum {
                        Ok(hash_emnum) => {
                            let data: BTreeMap<String, Value> = [
                                ("user_id".into(), id.clone().into()),
                                ("emnum".into(), hash_emnum.clone().into()),
                                ("username".into(), body.username.clone().into()),
                                ("password".into(), hash_pass.clone().into()),
                                ("sex".into(), format!("{:?}", body.sex).into()),
                            ]
                            .into();

                            let var: BTreeMap<String, Value> =
                                [("data".into(), data.into())].into();

                            let resul = ds.execute(&sql, ses, Some(var), false).await;

                            match resul {
                                Ok(_) => {
                                    let claim: Claims = Claims {
                                        id,
                                        exp,
                                        emnum: hash_emnum.clone(),
                                        sex: body.sex.clone(),
                                        username: body.username.clone(),
                                        password: hash_pass.clone(),
                                    };

                                    let token: String = encode(
                                        &Header::default(),
                                        &claim,
                                        &EncodingKey::from_secret(secret.as_str().as_ref()),
                                    )
                                    .unwrap();

                                    HttpResponse::Ok().json(EncodeResponse {
                                        message: String::from("success"),
                                        token,
                                    })
                                }
                                Err(_) => HttpResponse::InternalServerError().json(Response {
                                    message: "Something Went Wrong!".to_string(),
                                }),
                            }
                        }
                        Err(e) => HttpResponse::BadRequest().json(Response {
                            message: e.to_string(),
                        }),
                    }
                }
                Err(e) => HttpResponse::BadRequest().json(Response {
                    message: e.to_string(),
                }),
            }
        }
        None => HttpResponse::BadRequest().json(Response {
            message: "WTF NONE?".to_string(),
        }),
    }
}
