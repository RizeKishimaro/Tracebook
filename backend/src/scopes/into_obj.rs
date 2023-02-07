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

pub fn get_value(resul: Vec<Response>) -> Result<surrealdb::sql::Object> {
    let resul = into_obj(resul)?.into_iter().next().unwrap().unwrap();
    Ok(resul)
}

pub fn obj_str(obj: surrealdb::sql::Object, key: String) -> Result<String> {
    let resul = obj.get(&key).unwrap().to_string();

    Ok(resul)
}
