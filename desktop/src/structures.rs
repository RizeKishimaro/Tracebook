use eframe::egui::{Color32, Context, Window};
use serde::{Deserialize, Serialize};

pub const WARN: Color32 = Color32::from_rgb(255, 121, 0);
pub const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
pub const PADDING: f32 = 5.;

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
    pub nf: NfPage,
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
    pub nameerrvisi: bool,
    pub nameava: Option<bool>,
    pub namechd: bool,
    pub passerrr: String,
    pub passerrvivi: bool,
    pub errwin: bool,
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
    pub errwin: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NfPage {
    pub caption: String,
    pub image: Option<String>,
    pub type_supt: Option<bool>,
    pub cont_type: String,
    pub req_data: Vec<u8>,
    pub err_win: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccInfo {
    pub authd: bool,
    pub token: String,
}

#[derive(Debug, Default)]
pub struct MApp {
    pub pages: Pages,
    pub login: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostResp {
    pub val: Vec<Post>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImgBytes {
    pub value: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub fullname: String,
    pub caption: String,
    pub owner: String,
    pub image: String,
    pub id: String,
    pub down: u32,
    pub up: u32,
}

pub fn err_win(ctx: &Context, state: &mut bool, errname: &str) {
    Window::new("Error").open(state).show(ctx, |ui| {
        ui.label(errname);
    });
}

impl Default for AccInfo {
    fn default() -> Self {
        AccInfo {
            authd: false,
            token: "".into(),
        }
    }
}

impl Default for NfPage {
    fn default() -> Self {
        NfPage {
            image: None,
            type_supt: None,
            caption: String::new(),
            cont_type: String::new(),
            req_data: Vec::new(),
            err_win: false,
        }
    }
}
