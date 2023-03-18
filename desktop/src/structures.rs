use eframe::egui::{Color32, Context, Window};
use serde::{Deserialize, Serialize};

pub const WARN: Color32 = Color32::from_rgb(255, 121, 0);

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

pub fn err_win(ctx: &Context, state: &mut bool) {
    Window::new("Error").open(state).show(ctx, |ui| {
        ui.label("Something went wrong please Try again later!");
    });
}
