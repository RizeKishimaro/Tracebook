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
    argon_data: web::Data<Vec<String>>,
    blabla: Config<'_>,
) -> HttpResponse {
    let body_resul = body.user.as_ref();
    match body_resul {
        Some(body) => {
            let id = format!("{}{}", random::<u32>(), body.username.clone());
            let exp = (Utc::now() + Duration::days(365)).timestamp() as usize;
            let sql = format!("CREATE user:{id} CONTENT $data");

            let (mut argon_sec, mut argon_ad, mut argon_salt) = (
                argon_data[0].clone(),
                argon_data[1].clone(),
                argon_data[2].clone(),
            );

            let extra_sec = format!(
                "{}{}{}",
                body.username.clone(),
                body.password.clone(),
                body.emnum.clone()
            );

            argon_sec.push_str(&extra_sec);
            argon_ad.push_str(&extra_sec);
            argon_salt.push_str(&extra_sec);

            let config = Config {
                ad: argon_ad.as_bytes(),
                hash_length: 256,
                lanes: 35,
                mem_cost: 99999,
                secret: argon_sec.as_bytes(),
                thread_mode: argon2::ThreadMode::Parallel,
                time_cost: 3,
                variant: argon2::Variant::Argon2i,
                version: argon2::Version::Version13,
            };

            let hashed_pass = hash_encoded(
                body.password.clone().as_bytes(),
                argon_salt.as_bytes(),
                &config,
            );

            match hashed_pass {
                Ok(hash_pass) => {
                    let data: BTreeMap<String, Value> = [
                        ("user_id".into(), id.clone().into()),
                        ("emnum".into(), body.emnum.clone().into()),
                        ("username".into(), hash_pass.into()),
                        ("password".into(), body.password.clone().into()),
                        ("sex".into(), format!("{:?}", body.sex).into()),
                    ]
                    .into();

                    let var: BTreeMap<String, Value> = [("data".into(), data.into())].into();

                    let resul = ds.execute(&sql, ses, Some(var), false).await;

                    match resul {
                        Ok(_) => {
                            let claim: Claims = Claims {
                                id,
                                exp,
                                emnum: body.emnum.clone(),
                                sex: body.sex.clone(),
                                username: body.username.clone(),
                                password: body.password.clone(),
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
