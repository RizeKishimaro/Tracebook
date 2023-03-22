use std::{collections::BTreeMap, path::Path};

use crate::{
    extra::into_obj::get_value,
    structures::{Claims, PostResp, Resp, DB},
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use mime::{IMAGE_JPEG, IMAGE_PNG};
use rand::random;
use surrealdb::sql::Value;
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

use actix_multipart::Multipart;
use actix_web::{web, HttpResponse};
use futures_util::TryStreamExt as _;

use uuid::Uuid;

pub async fn post_upload(
    mut payload: Multipart,
    secret: web::Data<String>,
    (ds, ses): &DB,
) -> HttpResponse {
    let dir = "./user_uploaded_assets";
    let mut token = String::new();
    let mut file_data = Vec::<u8>::new();
    let post_id = random::<u32>();
    let mut name = String::default();
    let mut caption = Some(String::default());

    if !Path::new(dir).exists() {
        match fs::create_dir(dir).await {
            Ok(_) => {}
            Err(_) => {
                return HttpResponse::InternalServerError().json(Resp {
                    message: "Error in User assets file creating!".into(),
                    value: "Just panic!".into(),
                });
            }
        }
    }

    let file_type = [IMAGE_PNG, IMAGE_JPEG];
    while let Some(mut field) = payload.try_next().await.unwrap() {
        let content_disposition = field.content_disposition();

        let field_name = content_disposition.get_name().unwrap();

        match field_name {
            "file" => {
                let filet = field.content_type();
                if filet.is_none() {
                    return HttpResponse::BadRequest().json(Resp {
                        message: "File Type Error!".into(),
                        value: "Just panic!".into(),
                    });
                }
                if !file_type.contains(filet.unwrap()) {
                    println!("Ft err");
                    return HttpResponse::InternalServerError().json(Resp {
                        message: "File Type Not Supported!".into(),
                        value: "Just panic!".into(),
                    });
                }
                name = format!(
                    "{}/{}-{}",
                    dir,
                    Uuid::new_v4(),
                    content_disposition.get_filename().unwrap()
                );
                while let Some(chunk) = field.try_next().await.unwrap() {
                    file_data.extend_from_slice(&chunk);
                }
            }
            "token" => match field.try_next().await {
                Ok(bytes) => match bytes {
                    Some(bytess) => match String::from_utf8(bytess.to_vec()) {
                        Ok(tok) => token = tok,
                        Err(_) => {
                            return HttpResponse::NotAcceptable().json(Resp {
                                message: "No Token?".into(),
                                value: "Just panic!".into(),
                            })
                        }
                    },
                    None => {
                        return HttpResponse::NotAcceptable().json(Resp {
                            message: "No Token?".into(),
                            value: "Just panic!".into(),
                        })
                    }
                },
                Err(_) => {
                    return HttpResponse::NotAcceptable().json(Resp {
                        message: "No Token?".into(),
                        value: "Just panic!".into(),
                    })
                }
            },
            "caption" => {
                let bytes = field.try_next().await.unwrap();
                caption = String::from_utf8(bytes.unwrap_or("".as_bytes().into()).to_vec()).ok();
            }
            _ => {}
        }
    }

    match token.clone().is_empty() {
        false => {
            let claim = decode::<Claims>(
                &token,
                &DecodingKey::from_secret(secret.as_str().as_ref()),
                &Validation::new(jsonwebtoken::Algorithm::HS256),
            );
            match claim {
                Ok(claims) => {
                    let user_check_sql =
                        format!("SELECT password FROM user:{};", claims.claims.username);
                    match ds.execute(&user_check_sql, ses, None, false).await {
                        Ok(resp) => match get_value(resp) {
                            Ok(obj) => match obj.get("password") {
                                Some(_) => {
                                    if let false = file_data.is_empty() {
                                        match File::create(name.clone()).await {
                                            Ok(mut file_cre) => {
                                                match file_cre.write_all(&file_data).await {
                                                    Ok(_) => {}
                                                    Err(_) => {
                                                        return HttpResponse::InternalServerError().json(Resp {message: "Error in writing bytes as file!".into(), value: "Just panic!".into()});
                                                    }
                                                }
                                            }
                                            Err(_) => {
                                                return HttpResponse::InternalServerError().json(
                                                    Resp {
                                                        message: "Error in File creating!".into(),
                                                        value: "Just panic!".into(),
                                                    },
                                                );
                                            }
                                        };
                                    }

                                    let upload_post_sql =
                                        format!("CREATE post:{} CONTENT $data;", post_id);

                                    let data: BTreeMap<String, Value> = [
                                        (
                                            "owner".into(),
                                            format!("user:{}", claims.claims.username.clone())
                                                .into(),
                                        ),
                                        (
                                            "caption".into(),
                                            caption.clone().unwrap_or("".into()).into(),
                                        ),
                                        ("image".into(), name.clone().into()),
                                        ("up".into(), 0.into()),
                                        ("down".into(), 0.into()),
                                    ]
                                    .into();

                                    let var: BTreeMap<String, Value> =
                                        [("data".into(), data.into())].into();

                                    match ds.execute(&upload_post_sql, ses, Some(var), false).await
                                    {
                                        Ok(resp) => {
                                            match get_value(resp) {
                                                Ok(_) => {
                                                    let upd_user =
                                                                format!("UPDATE user:{} SET posts += [post:{post_id}]", claims.claims.username.clone());

                                                    match ds.execute(&upd_user, ses, None, false).await {
                                                                Ok(resp) => {
                                                                        match get_value(resp) {
                                                                            Ok(_) => {
                                                                                HttpResponse::Ok().json(PostResp {
                                                                                    username: claims.claims.username.clone(),
                                                                                    caption: caption.unwrap_or("".into()).clone(),
                                                                                    images: name,
                                                                                    upvote: 0,
                                                                                    dwvote: 0,

                                                                                })
                                                                            },
                                                                            Err(_) => HttpResponse::InternalServerError().json(Resp {message: "Error in Storing post to user!".into(), value: "Just panic!".into()})
                                                                        }
                                                                    },
                                                                Err(_) => HttpResponse::InternalServerError().json(Resp {message: "Error in Storing post to user!".into(), value: "Just panic!".into()})
                                                            }
                                                }
                                                Err(_) => {
                                                    HttpResponse::InternalServerError().json(Resp {
                                                        message: "Error in Storing post!".into(),
                                                        value: "Just panic!".into(),
                                                    })
                                                }
                                            }
                                        }
                                        Err(_) => HttpResponse::InternalServerError().json(Resp {
                                            message: "Error in Storing post!".into(),
                                            value: "Just panic!".into(),
                                        }),
                                    }
                                }
                                None => HttpResponse::InternalServerError().json(Resp {
                                    message: "No password?".into(),
                                    value: "Just panic!".into(),
                                }),
                            },

                            Err(e) => HttpResponse::InternalServerError().json(Resp {
                                message: e,
                                value: "Just panic!".into(),
                            }),
                        },
                        Err(_) => HttpResponse::InternalServerError().json(Resp {
                            message: "Error in User Select!".into(),
                            value: "Just panic!".into(),
                        }),
                    }
                }
                Err(_) => HttpResponse::InternalServerError().json(Resp {
                    message: "Error in decoding jwt!".into(),
                    value: "Just panic!".into(),
                }),
            }
        }

        true => HttpResponse::NotAcceptable().json(Resp {
            message: "No Token?".into(),
            value: "Just panic!".into(),
        }),
    }
}
