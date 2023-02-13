use std::collections::BTreeMap;

use actix_web::{web, HttpResponse};
use jsonwebtoken::{decode, errors::Error, DecodingKey, TokenData, Validation};
use rand::random;
use surrealdb::sql::Value;

use crate::{
    extra::into_obj::{get_value, obj_str},
    scopes::user::{Claims, Response, DB},
};

use super::upload_sc::{Links, Model, ResponsePost};

pub async fn post(
    (ds, ses): &DB,
    model: web::Json<Model>,
    secret: web::Data<String>,
) -> HttpResponse {
    let post_id = random::<u64>();
    let user_info: Result<TokenData<Claims>, Error> = decode(
        &model.user_token,
        &DecodingKey::from_secret(secret.as_str().as_ref()),
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    );
    let sql = format!("CREATE post:{post_id} CONTENT $data;");

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
    let post = ds.execute(&sql, ses, Some(var), false).await;

    match post {
        Ok(obj) => {
            let resul = get_value(obj);

            match resul {
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

                    let resul = ds.execute(&user_sql, ses, None, false).await;

                    println!("{vec_str_resul:?}");

                    match resul {
                        Ok(_) => HttpResponse::Ok().json(ResponsePost {
                            post_id: vec_str_resul[0].parse().unwrap_or(0o712404404403),
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
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(Response {
            message: e.to_string(),
        }),
    }
}

fn match_links(links: Links) -> String {
    match links {
        Links::None => "None".to_string(),
        Links::Links(links) => format!("{links:?}"),
    }
}
