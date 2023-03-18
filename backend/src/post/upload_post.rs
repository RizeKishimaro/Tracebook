use std::path::Path;

use crate::structures::Resp;
use mime::{IMAGE_JPEG, IMAGE_PNG};
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

use actix_multipart::Multipart;
use actix_web::{web, HttpResponse};
use futures_util::TryStreamExt as _;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Less {
    less: String,
}

pub async fn post_upload(mut payload: Multipart) -> HttpResponse {
    let dir = "./user_uploaded_assets";
    let mut text_op = Some("".to_owned());
    let mut file_data = Vec::<u8>::new();
    let mut name = String::default();
    if !Path::new(dir).exists() {
        match fs::create_dir(dir).await {
            Ok(_) => {}
            Err(_) => {
                println!("Craft Err");
                return HttpResponse::InternalServerError().json(Resp {
                    message: "Error in User assets file creating!".into(),
                    value: "Just panic!".into(),
                });
            }
        }
    }

    let file_type = [IMAGE_PNG, IMAGE_JPEG];
    while let Some(mut field) = payload.try_next().await.unwrap() {
        println!("{field:?}");
        let content_disposition = field.content_disposition();

        let field_name = content_disposition.get_name().unwrap();
        println!("{field_name}");
        match field_name {
            "file" => {
                let filet = field.content_type();
                if filet.is_none() {
                    continue;
                }
                if !file_type.contains(&filet.unwrap()) {
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
            "text" => {
                let bytes = field.try_next().await.unwrap();
                text_op = String::from_utf8(bytes.unwrap().to_vec()).ok();
            }

            _ => {}
        }
    }

    println!("{text_op:?}");
    if !(text_op.unwrap().as_str() == "Nope") {
        let mut file_create = File::create(name).await.unwrap();
        file_create.write_all(&file_data.as_slice()).await.unwrap();
    } else {
        println!("Nope err");
        return HttpResponse::InternalServerError().json(Resp {
            message: "Nope".into(),
            value: "Nope".into(),
        });
    }

    HttpResponse::Ok().json(Resp {
        message: "Uploaded!".into(),
        value: "Nice Posted!".into(),
    })
}
