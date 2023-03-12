use serde::{Deserialize, Serialize};

use crate::pages::login::LoginPage;

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum Sex {
    Male,
    Female,
    #[default]
    Custom,
}

#[derive(Debug, Default)]
pub struct Pages {
    pub login: LoginPage,
}
