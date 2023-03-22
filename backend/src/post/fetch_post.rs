use actix_web::HttpResponse;

use crate::structures::{Resp, VDB};

pub async fn fetch_post() -> HttpResponse {
    let (ds, ses) = VDB.get().await;
    let fetch_sql = format!("SELECT * FROM post;");
    match ds.execute(&fetch_sql, ses, None, false).await {
        Ok(resp) => HttpResponse::Ok().json(Resp {
            message: "Test!".into(),
            value: format!("{resp:?}"),
        }),
        Err(_) => HttpResponse::InternalServerError().json(Resp {
            message: "Error in post fetching from db!".into(),
            value: "Just panic!".into(),
        }),
    }
}
