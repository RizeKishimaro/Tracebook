use anyhow::{anyhow, Result};
use surrealdb::sql::Value::{Array, Object};
use surrealdb::Response;

pub fn into_obj(
    ress: Vec<Response>,
) -> Result<impl Iterator<Item = Result<surrealdb::sql::Object>>> {
    let res = ress
        .into_iter()
        .next()
        .map(|rp| rp.result)
        .transpose()
        .unwrap();

    match res {
        Some(Array(arr)) => {
            let resul = arr.into_iter().map(|v| match v {
                Object(obj) => Ok(obj),
                _ => Err(anyhow!("Invalid record!")),
            });
            Ok(resul)
        }
        _ => Err(anyhow!("No record found!")),
    }
}

pub fn get_value(resul: Vec<Response>) -> Result<surrealdb::sql::Object, String> {
    let resul = into_obj(resul).unwrap().next();

    match resul {
        Some(s) => match s {
            Ok(resul) => Ok(resul),
            Err(e) => Err(e.to_string()),
        },

        None => Err("No Result!".to_string()),
    }
}

pub fn get_vec_value(resul: Vec<Response>) -> Vec<surrealdb::sql::Object> {
    into_obj(resul).unwrap().map(|resp| resp.unwrap()).collect()
}

pub fn _obj_str(obj: surrealdb::sql::Object, keys: Vec<String>) -> Vec<String> {
    keys.iter()
        .map(|key| obj.get(key).unwrap().to_string())
        .collect()
}
