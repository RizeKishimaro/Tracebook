use anyhow::anyhow;
use surrealdb::sql::Value::{Array, Object};
use surrealdb::Response;

fn into_obj(
    ress: Vec<Response>,
) -> Result<impl Iterator<Item = Result<surrealdb::sql::Object, anyhow::Error>>, anyhow::Error> {
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
