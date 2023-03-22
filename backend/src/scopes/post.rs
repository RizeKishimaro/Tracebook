use actix_multipart::Multipart;
use actix_web::{web, HttpResponse, Scope};

use crate::{
    post::upload_post::post_upload,
    structures::{Resp, VDB},
};

pub fn post_scope() -> Scope {
    web::scope("/post").route("{method}", web::post().to(post_branch))
}

pub async fn post_branch(
    payload: Multipart,
    secret: web::Data<String>,
    method: web::Path<String>,
) -> HttpResponse {
    match method.as_str() {
        "upload_post" => post_upload(payload, secret, VDB.get().await).await,
        _ => HttpResponse::NotFound().json(Resp {
            message: "Nope No Method Found!".into(),
            value: "Bruh!".into(),
        }),
    }
}
