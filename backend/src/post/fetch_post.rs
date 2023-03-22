use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Object;

#[derive(Debug, Serialize, Deserialize)]
struct TResp {
    val: Vec<Object>,
}

use crate::{
    extra::into_obj::get_vec_value,
    structures::{Resp, VDB},
};

pub async fn fetch_post() -> HttpResponse {
    let (ds, ses) = VDB.get().await;
    let fetch_sql = format!("SELECT * FROM post LIMIT 10;");
    match ds.execute(&fetch_sql, ses, None, false).await {
        Ok(resp) => HttpResponse::Ok().json(TResp {
            val: get_vec_value(resp),
        }),
        Err(_) => HttpResponse::InternalServerError().json(Resp {
            message: "Error in post fetching from db!".into(),
            value: "Just panic!".into(),
        }),
    }
}
