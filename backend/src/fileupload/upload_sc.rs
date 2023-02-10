use actix_web::{web, Scope};

pub fn post_scope() -> Scope {
    web::scope("/post")
}
