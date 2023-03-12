use serde::{Deserialize, Serialize};
use serde_json::from_str;

#[derive(Serialize, Deserialize, Debug)]
pub enum PostType {
    Global,
    OnlyMe,
    Friends,
}

impl From<String> for PostType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Global" => PostType::Global,
            "OnlyMe" => PostType::OnlyMe,
            "Friends" => PostType::Friends,
            _ => PostType::Global,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Links {
    Links(Vec<String>),
    None(String),
}

impl From<String> for Links {
    fn from(value: String) -> Self {
        let value = &value[1..value.len() - 1];
        match value {
            "--! None" => Links::None("--! None".to_string()),
            _ => Links::Links(from_str(value).unwrap()),
        }
    }
}
