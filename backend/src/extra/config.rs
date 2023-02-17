use argon2::Config;
use dotenvy::var;

pub fn encrypt_config(extra: String) -> Config {
    let vec_vars = vec!["SECRET_ARGON", "SALT", "AD"];
    let resul = vec_vars(vec_vars);
}

pub fn vec_vars(vec_vars: Vec<&str>) -> Vec<String> {
    vec_vars.iter().map(|v| var(v).unwrap()).collect()
}
