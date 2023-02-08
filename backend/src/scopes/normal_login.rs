use actix_web::{web, HttpResponse};

use super::user::{Info, DB};

pub async fn login((ds, ses): &DB, body: web::Json<Info>) -> HttpResponse {
    let sql = format!(
        "SELECT * FROM user WHERE emnum = \"{}\" AND username = \"{}\" AND password = \"{}\" AND sex = \"{}\";",
        body.emnum.clone(),
        body.username.clone(),
        body.password.clone(),
        body.sex.clone()
    );

    let resul = ds.execute(&sql, ses, None, false).await.unwrap();

    println!("{resul:?}");

    HttpResponse::Ok().await.unwrap()
}
