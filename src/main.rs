use std::path::PathBuf;

use actix_files::NamedFile;
use actix_web::http::StatusCode;
use actix_web::HttpRequest;
use actix_web::{middleware::Logger, web, App, Error, HttpResponse, HttpServer};

struct Pages;

impl Pages {
    async fn home() -> Result<HttpResponse, Error> {
        Ok(HttpResponse::build(StatusCode::OK)
            .content_type("text/html; charset=utf-8")
            .body(include_str!("../index.html")))
    }

    async fn profile() -> Result<HttpResponse, Error> {
        Ok(HttpResponse::build(StatusCode::OK)
            .content_type("text/html; charset=utf-8")
            .body(include_str!("../profile.html")))
    }
}

async fn get_file(req: HttpRequest) -> HttpResponse {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    match NamedFile::open(path) {
        Ok(nm) => nm.into_response(&req),
        _ => HttpResponse::NotFound().body(include_str!("../404.html")),
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(Pages::home))
            .route("/profile", web::get().to(Pages::profile))
            .route("/{filename:.*}", web::get().to(get_file))
            .wrap(Logger::default())
    })
    .bind("127.0.0.1:8888")?
    .run()
    .await
}
