use argon2::Config;
use async_once::AsyncOnce;
use dotenvy::var;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use surrealdb::{Datastore, Session};
use tokio::runtime::Runtime;

lazy_static! {
    pub static ref ARGON_DT: (String, String, String) = (
        var("SECRETARGON").unwrap(),
        var("AD").unwrap(),
        var("SALT").unwrap()
    );
    pub static ref ARG_CONFIG: Config<'static> = Config {
        ad: ARGON_DT.1.as_bytes(),
        hash_length: 256,
        lanes: 35,
        mem_cost: 99999,
        secret: ARGON_DT.0.as_bytes(),
        thread_mode: argon2::ThreadMode::Parallel,
        time_cost: 3,
        variant: argon2::Variant::Argon2i,
        version: argon2::Version::Version13,
    };
    pub static ref VDB: AsyncOnce<DB> = {
        AsyncOnce::new(async {
            (
                Datastore::new("file://tracebook.db").await.unwrap(),
                Session::for_db("trace", "book"),
            )
        })
    };
}
pub type DB = (Datastore, Session);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Sex {
    Male,
    Female,
    Custom,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Resp {
    pub message: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Signup {
    pub username: String,
    pub password: String,
    pub fullname: String,
    pub sex: Sex,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReqInfo {
    pub token: Option<String>,
    pub signup: Option<Signup>,
    pub login: Option<Login>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Claims {
    pub username: String,
    pub password: String,
    pub exp: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChInfo {
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostModel {
    pub text: String,
    pub images: Option<Vec<String>>,
    pub videos: Option<Vec<String>>,
    pub token: String,
}
