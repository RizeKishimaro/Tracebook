use dotenvy::var;

pub fn vec_vars(vec_vars: Vec<&str>) -> Vec<String> {
    vec_vars.iter().map(|v| var(v).unwrap()).collect()
}
