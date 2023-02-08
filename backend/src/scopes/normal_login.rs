use actix_web::{web, HttpResponse};

use super::user::{Info, DB};

pub async fn login((ds, ses): &DB, body: web::Json<Info>) -> HttpResponse {
    let sql = format!("SELECT * FROM user WHERE (emnum = {});", body.emnum.clone());

    let resul = ds.execute(&sql, ses, None, false).await.unwrap();

    println!("{resul:?}");

    HttpResponse::Ok().await.unwrap()
}
