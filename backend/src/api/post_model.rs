use actix_web::{
    get,
    http::header::{self, HeaderMap, HeaderValue},
    HttpResponse, Responder,
};
use std::{fs, path::Path};
#[get("/post")]
pub async fn file_upload() -> impl Responder {
    let mut map = HeaderMap::new();
    map.insert(header::LOCATION, HeaderValue::from_static("index.php"));
    let _media_files = "Media";
    let json_folder_name = "Server_Logs";
    if Path::new(json_folder_name).is_dir() {
        let json_root_files = fs::read_dir(json_folder_name).unwrap();
        for i in json_root_files {
            let read_file_open = fs::read_to_string(i.unwrap().path().to_str().unwrap()).unwrap();

            let data: serde_json::Value = serde_json::from_str(&read_file_open).unwrap();

            if true {
                return HttpResponse::Ok().json(data);
            }
        }
        return HttpResponse::InternalServerError().body("Error");
    } else {
        let data: serde_json::Value = serde_json::from_str("Error").unwrap();
        return HttpResponse::Ok().json(data);
    }
}
