use actix_web::{web, HttpResponse, Scope};

pub fn post_scope() -> Scope {
    web::scope("/post").route("{method}", web::post().to(post_branch))
}

pub async fn post_branch() -> HttpResponse {
    todo!()
}
