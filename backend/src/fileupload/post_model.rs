use std::collections::BTreeMap;

use actix_web::{web, HttpResponse};
use argon2::hash_encoded;
use jsonwebtoken::{decode, errors::Error, DecodingKey, TokenData, Validation};
use rand::random;
use surrealdb::sql::Value;

use crate::{
    extra::into_obj::{get_value, obj_str},
    structures::auth_struct::*,
    structures::{
        post_enum::{Links, PostType},
        post_struct::{Model, ResponsePost},
    },
};

pub async fn post(
    (ds, ses): &DB,
    model: web::Json<Model>,
    secret: web::Data<String>,
) -> HttpResponse {
    let post_id = random::<u32>();
    let user_info: Result<TokenData<Claims>, Error> = decode(
        &model.user_token,
        &DecodingKey::from_secret(secret.as_str().as_ref()),
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    );
    let sql = format!("CREATE post:{post_id} CONTENT $data;");

    let data_def = match model.post_type {
        PostType::Global => Ok(vec![
            model.text.clone().unwrap_or("--! None".to_string()),
            match_links(model.images.clone()),
            match_links(model.videos.clone()),
        ]),
        PostType::OnlyMe | PostType::Friends => encrypt_func(
            secret,
            vec![
                model.text.clone().unwrap_or("--! None".to_string()),
                match_links(model.images.clone()),
                match_links(model.videos.clone()),
            ],
        ),
    };

    match data_def {
        Ok(data_defd) => {
            let data: BTreeMap<String, Value> = [
                ("post_id".into(), post_id.into()),
                ("post_type".into(), format!("{:?}", model.post_type).into()),
                ("text".into(), model.text.clone().into()),
                ("images".into(), match_links(model.images.clone()).into()),
                ("videos".into(), match_links(model.videos.clone()).into()),
                (
                    "user_poster".into(),
                    format!("user:{}", user_info.unwrap().claims.id).into(),
                ),
            ]
            .into();

            let var: BTreeMap<String, Value> = [("data".into(), data.into())].into();
            match ds.execute(&sql, ses, Some(var), false).await {
                Ok(obj) => match get_value(obj) {
                    Ok(v) => {
                        let keys = vec![
                            "post_id".to_string(),
                            "post_type".to_string(),
                            "text".to_string(),
                            "images".to_string(),
                            "videos".to_string(),
                            "user_poster".to_string(),
                        ];

                        let vec_str_resul = obj_str(v, keys);

                        let user_sql = format!(
                            "UPDATE {} SET posts += ['post:{}']",
                            vec_str_resul[5], vec_str_resul[0]
                        );

                        match ds.execute(&user_sql, ses, None, false).await {
                            Ok(_) => HttpResponse::Ok().json(ResponsePost {
                                post_id: vec_str_resul[0].parse().unwrap_or(0o712404404),
                                post_type: vec_str_resul[1].clone().into(),
                                text: vec_str_resul[2].clone().into(),
                                images: vec_str_resul[3].clone().into(),
                                videos: vec_str_resul[4].clone().into(),
                            }),
                            Err(e) => HttpResponse::InternalServerError().json(Response {
                                message: e.to_string(),
                            }),
                        }
                    }
                    Err(e) => HttpResponse::InternalServerError().json(Response { message: e }),
                },
                Err(e) => HttpResponse::InternalServerError().json(Response {
                    message: e.to_string(),
                }),
            }
        }
        Err(_) => HttpResponse::InternalServerError().json(Response {
            message: "I don't fucking know why!".to_string(),
        }),
    }
}

pub fn match_links(links: Links) -> String {
    match links {
        Links::None(_) => "--! None".to_string(),
        Links::Links(links) => format!("{links:?}"),
    }
}

pub fn encrypt_func(
    secret: web::Data<String>,
    vec_data: Vec<String>,
) -> Result<Vec<String>, String> {
    let check_data: Vec<String> = vec_data
        .iter()
        .map(|v| match v.as_str() {
            "--! None" => "--! None".to_string(),
            _ => match hash_encoded(v.as_bytes(), secret.as_bytes(), &argon2::Config::default()) {
                Ok(d) => d,
                Err(_) => "--! Error".to_string(),
            },
        })
        .collect();

    let check = check_data
        .iter()
        .filter(|x| x.to_string() == "--! Error".to_string())
        .count();

    if check > 0 {
        Err("Error".to_string())
    } else {
        Ok(check_data)
    }
}
