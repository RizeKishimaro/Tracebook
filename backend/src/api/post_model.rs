use actix_web::{
    get,
    http::header::{self, HeaderMap, HeaderValue},
    HttpResponse, Responder,
};
use serde_json::Value;
use std::{fs, path::Path};

fn merge(a: &mut Value, b: Value) {
    match (a, b) {
        (a @ &mut Value::Object(_), Value::Object(b)) => {
            let a = a.as_object_mut().unwrap();
            for (k, v) in b {
                merge(a.entry(k).or_insert(Value::Null), v);
            }
        }
        (a, b) => *a = b,
    }
}

#[get("/post")]
pub async fn file_upload() -> impl Responder {
    let mut map = HeaderMap::new();
    map.insert(header::LOCATION, HeaderValue::from_static("index.php"));
    let _media_files = "Media";
    let json_folder_name = "Server_Logs";
    let mut data = Value::default();
    if Path::new(json_folder_name).is_dir() {
        let json_root_files = fs::read_dir(json_folder_name).unwrap();
        for i in json_root_files {
            let read_file_open = fs::read_to_string(i.unwrap().path().to_str().unwrap()).unwrap();

            let json_data: Value = serde_json::from_str(&read_file_open).unwrap();
            merge(&mut data, json_data);
            if true {
                return HttpResponse::Ok().json(data);
            }
        }

        HttpResponse::Ok().json(data)
    } else {
        let data: serde_json::Value = serde_json::from_str("Error").unwrap();
        return HttpResponse::Ok().json(data);
    }
}
