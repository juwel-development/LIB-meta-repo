use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct App {
    pub dir: String,
    pub git: String,
}

impl App {
    pub fn get_initial_config() -> Self {
        Self {
            dir: "".to_string(),
            git: "".to_string(),
        }
    }
}