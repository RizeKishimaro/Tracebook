use crate::extractors::auth_token::AuthToken;
use crate::scopes::encode::encode_token;
use actix_web::{web, Scope};

pub fn user_scope() -> Scope {
    web::scope("/user").route("encode-token", web::post().to(encode_token))
}
