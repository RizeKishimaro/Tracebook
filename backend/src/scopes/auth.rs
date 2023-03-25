use crate::{
    auth::{login::login, signup::signup},
    structures::{ReqInfo, Resp, VDB},
};
use actix_web::{web, HttpResponse, Scope};

pub fn auth_scope() -> Scope {
    web::scope("/auth").route("{method}", web::post().to(auth_branch))
}

pub async fn auth_branch(
    method: web::Path<String>,
    secret: web::Data<String>,
    info: web::Json<ReqInfo>,
) -> HttpResponse {
    eprintln!("{:?} {:?}", secret, info);
    match method.as_str() {
        "signup" => signup(VDB.get().await, info, secret).await,
        "login" => login(VDB.get().await, info, secret).await,
        _ => HttpResponse::NotFound().json(Resp {
            message: "Method not Found!".into(),
            value: "Just panic!".into(),
        }),
    }
}
