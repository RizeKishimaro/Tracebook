use actix_web::{web, HttpResponse};

use crate::{
    extra::into_obj::get_value,
    structures::{ChInfo, Resp, VDB},
};

pub async fn ch_name_fnc(info: web::Json<ChInfo>) -> HttpResponse {
    let (ds, ses) = VDB.get().await;

    let ch_sql = format!("SELECT * FROM user:{}", &info.username);

    match ds.execute(&ch_sql, ses, None, false).await {
        Ok(resp_resul) => match get_value(resp_resul) {
            Ok(_) => HttpResponse::BadRequest().json(Resp {
                message: "User Already Exit!".into(),
                value: "Just panic!".into(),
            }),
            Err(_) => HttpResponse::Ok().json(Resp {
                message: "Name is Ok!".into(),
                value: "Go ahead!".into(),
            }),
        },
        Err(_) => HttpResponse::InternalServerError().json(Resp {
            message: "Error in User Searching!".into(),
            value: "Just panic!".into(),
        }),
    }
}
