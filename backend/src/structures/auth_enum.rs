use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Sex {
    Male,
    Female,
    Intersex,
    Nonbin,
    Genderqueer,
    Twospirit,
    Androgynous,
    Bigender,
    Thirdgender,
    Notshow,
}
