use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Sex {
    Male,
    Female,
    Custom,
}
