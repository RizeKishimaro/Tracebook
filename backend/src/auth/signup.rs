use std::collections::BTreeMap;

use crate::{
    extra::into_obj::get_value,
    structures::{Claims, ReqInfo, Resp, ARGON_DT, ARG_CONFIG, DB},
};
use actix_web::{web, HttpResponse};
use argon2::hash_encoded;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use surrealdb::sql::Value;

pub async fn signup(
    (ds, ses): &DB,
    info: web::Json<ReqInfo>,
    secret: web::Data<String>,
) -> HttpResponse {
    match info.signup.as_ref() {
        Some(u_info) => {
            let name_check_sql = format!("SELECT * FROM user:{}", u_info.username);
            let exp = (Utc::now() + Duration::days(9876)).timestamp() as usize;

            let resul = ds.execute(&name_check_sql, ses, None, false).await;

            match resul {
                Ok(v_resul) => {
                    if let Ok(_) = get_value(v_resul) {
                        return HttpResponse::BadRequest().json(Resp {
                            message: "Username Already exits!".to_string(),
                            value: "Just panic!".to_string(),
                        });
                    }

                    let create_user_sql = format!("CREATE user:{} CONTENT $data", u_info.username);

                    let hashed_pass = hash_encoded(
                        u_info.password.as_bytes(),
                        ARGON_DT.2.as_bytes(),
                        &ARG_CONFIG,
                    );

                    match hashed_pass {
                        Ok(hash_pass) => {
                            let data: BTreeMap<String, Value> = [
                                ("username".into(), u_info.username.clone().into()),
                                ("password".into(), hash_pass.clone().into()),
                                ("fullname".into(), u_info.fullname.clone().into()),
                            ]
                            .into();

                            let var: BTreeMap<String, Value> =
                                [("data".into(), data.into())].into();

                            let claims: Claims = Claims {
                                username: u_info.username.clone(),
                                password: hash_pass.clone(),
                                exp,
                            };

                            match ds.execute(&create_user_sql, ses, Some(var), false).await {
                                Ok(resul) => {
                                    if get_value(resul).is_err() {
                                        return HttpResponse::InternalServerError().json(Resp {
                                            message: "Error in User Creating!".into(),
                                            value: "Just panic!".into(),
                                        });
                                    }

                                    let token = encode(
                                        &Header::default(),
                                        &claims,
                                        &EncodingKey::from_secret(secret.as_str().as_ref()),
                                    );

                                    if let Ok(jwt) = token {
                                        return HttpResponse::Ok().json(Resp {
                                            message: "Authed!".into(),
                                            value: jwt,
                                        });
                                    }

                                    HttpResponse::InternalServerError().json(Resp {
                                        message: "Error in creating jwt!".into(),
                                        value: "Just panic!".into(),
                                    })
                                }
                                Err(_) => HttpResponse::InternalServerError().json(Resp {
                                    message: "Error in Creating User!".to_string(),
                                    value: "Just panic!".to_string(),
                                }),
                            }
                        }
                        Err(_) => HttpResponse::InternalServerError().json(Resp {
                            message: "Error in Password hashing!".to_string(),
                            value: "Just panic!".to_string(),
                        }),
                    }
                }
                Err(_) => HttpResponse::InternalServerError().json(Resp {
                    message: "Error in User checking!".to_string(),
                    value: "Just panic!".to_string(),
                }),
            }
        }
        None => HttpResponse::InternalServerError().json(Resp {
            message: "No User Info".to_string(),
            value: "Just panic!".to_string(),
        }),
    }
}
