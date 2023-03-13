use serde::{Deserialize, Serialize};

type CLR = (u8, u8, u8);
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum Sex {
    Male,
    Female,
    #[default]
    Custom,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct AuthResp {
    pub message: String,
    pub value: String,
}

#[derive(Debug, Default)]
pub struct Pages {
    pub signup: SignupPage,
    pub login: LoginPage,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SignupPage {
    pub fullname: String,
    pub fnamerrvisi: bool,
    pub username: String,
    pub password: String,
    pub pass: bool,
    pub sex: Sex,
    pub namerror: String,
    pub namerrclr: CLR,
    pub nameerrvisi: bool,
    pub nameava: Option<bool>,
    pub namechd: bool,
    pub passerrr: String,
    pub passerrclr: CLR,
    pub passerrvivi: bool,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct LoginPage {
    pub username: String,
    pub namerror: String,
    pub nameerrvisi: bool,
    pub password: String,
    pub passerrr: String,
    pub passerrvisi: bool,
    pub passshow: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AccInfo {
    pub authd: bool,
    pub token: String,
}

#[derive(Debug, Default)]
pub struct MApp {
    pub pages: Pages,
    pub login: bool,
}
