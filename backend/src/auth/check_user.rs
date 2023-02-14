use anyhow::Result;
use surrealdb::Response;

use crate::{
    extra::into_obj::{get_value, obj_str},
    scopes::user::Claims,
};

pub fn check_user(data: Claims, resul: Vec<Response>) -> Result<String, String> {
    let valus = get_value(resul);

    match valus {
        Ok(obj) => {
            let keys = vec![
                "user_id".into(),
                "username".into(),
                "password".into(),
                "emnum".into(),
                "sex".into(),
            ];

            let vec_resul = obj_str(obj, keys);

            let vec_resul = fucking_double_string_vec_to_one_string_vec(vec_resul);

            if vec_resul[0] == data.id
                && vec_resul[1] == data.username
                && vec_resul[2] == data.password
                && (&vec_resul[3][1..vec_resul[3].len() - 1]).to_string() == data.emnum
                && vec_resul[4] == format!("{:?}", data.sex)
            {
                Ok("OK".to_string())
            } else {
                Err("No user!".to_string())
            }
        }
        Err(e) => Err(e),
    }
}

pub fn fucking_double_string_vec_to_one_string_vec(svec: Vec<String>) -> Vec<String> {
    svec.iter().map(|s| s[1..s.len() - 1].to_string()).collect()
}
