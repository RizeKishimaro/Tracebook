use crate::{
    extra::into_obj::get_value,
    structures::auth_struct::{Claims, EncodeResponse, Info, Response, DB},
};
use actix_web::{web, HttpResponse};
use argon2::{hash_encoded, Config};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DbResp {
    pub message: String,
    pub db_resl: String,
}

pub async fn login(
    (ds, ses): &DB,
    body: web::Json<Info>,
    secret: web::Data<String>,
    argon_salt: String,
    argon_config: Config<'_>,
) -> HttpResponse {
    match body.user.as_ref() {
        Some(body) => {
            let hashed_pass = hash_encoded(
                body.password.clone().as_bytes(),
                argon_salt.clone().as_bytes(),
                &argon_config,
            );

            match hashed_pass {
                Ok(hash_pass) => {
                    let hashed_emnum = hash_encoded(
                        body.emnum.clone().as_bytes(),
                        argon_salt.clone().as_bytes(),
                        &argon_config,
                    );
                    match hashed_emnum {
                        Ok(hash_emnum) => {
                            let sql = format!(
        "SELECT * FROM user WHERE emnum = \"{}\" AND username = \"{}\" AND password = \"{}\" AND sex = \"{:?}\";",
        hash_emnum,
        body.username.clone(),
        hash_pass,
        body.sex.clone()
    );

                            match ds.execute(&sql, ses, None, true).await {
                                Ok(resp) => {
                                    let check = get_value(resp);

                                    match check {
                                        Ok(obj) => {
                                            let id = obj.get("user_id").unwrap().to_string();
                                            let id = id[1..id.len() - 1].to_string();
                                            let exp = (Utc::now() + Duration::days(365)).timestamp()
                                                as usize;

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

                                        Err(e) => {
                                            HttpResponse::BadRequest().json(Response { message: e })
                                        }
                                    }
                                }

                                Err(e) => HttpResponse::BadRequest().json(DbResp {
                                    message: "UnAuthed".to_string(),
                                    db_resl: e.to_string(),
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
