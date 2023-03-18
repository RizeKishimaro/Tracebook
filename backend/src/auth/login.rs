use actix_web::{web, HttpResponse};
use argon2::verify_encoded_ext;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::{
    extra::into_obj::get_value,
    structures::{Claims, ReqInfo, Resp, ARGON_DT, DB},
};

pub async fn login(
    (ds, ses): &DB,
    info: web::Json<ReqInfo>,
    secret: web::Data<String>,
) -> HttpResponse {
    match info.login.as_ref() {
        Some(u_info) => {
            let ch_user_sql = format!("SELECT * FROM user:{}", u_info.username);
            match ds.execute(&ch_user_sql, ses, None, false).await {
                Ok(resp_resul) => match get_value(resp_resul) {
                    Ok(obj) => match obj.get("password") {
                        Some(password) => {
                            let pass = password.to_string();
                            let pass = pass[1..pass.len() - 1].to_string();
                            match verify_encoded_ext(
                                &pass,
                                u_info.password.as_bytes(),
                                ARGON_DT.0.as_bytes(),
                                ARGON_DT.1.as_bytes(),
                            ) {
                                Ok(verf) => {
                                    if verf {
                                        let exp = (Utc::now() + Duration::days(9876)).timestamp()
                                            as usize;
                                        let claims = Claims {
                                            username: u_info.username.clone(),
                                            password: u_info.password.clone(),
                                            exp,
                                        };
                                        let jwt = encode(
                                            &Header::default(),
                                            &claims,
                                            &EncodingKey::from_secret(secret.as_str().as_ref()),
                                        );

                                        match jwt {
                                            Ok(token) => HttpResponse::Ok().json(Resp {
                                                message: "Authed".into(),
                                                value: token,
                                            }),
                                            Err(_) => {
                                                HttpResponse::InternalServerError().json(Resp {
                                                    message: "Error in Generating JWT!".into(),
                                                    value: "Just panic!".into(),
                                                })
                                            }
                                        }
                                    } else {
                                        HttpResponse::Unauthorized().json(Resp {
                                            message: "Wrong Password!".into(),
                                            value: "Just panic!".into(),
                                        })
                                    }
                                }
                                Err(_) => HttpResponse::InternalServerError().json(Resp {
                                    message: "Error in Verifying password!".into(),
                                    value: "Just panic!".into(),
                                }),
                            }
                        }
                        None => HttpResponse::InternalServerError().json(Resp {
                            message: "Error in Database password!".into(),
                            value: "Just panic!".into(),
                        }),
                    },
                    Err(_) => HttpResponse::BadRequest().json(Resp {
                        message: "User Not Found!".into(),
                        value: "Just panic!".into(),
                    }),
                },
                Err(_) => HttpResponse::InternalServerError().json(Resp {
                    message: "Error in checking user!".into(),
                    value: "Just panic!".into(),
                }),
            }
        }
        None => HttpResponse::InternalServerError().json(Resp {
            message: "Error in Request body!".into(),
            value: "Just panic!".into(),
        }),
    }
}
