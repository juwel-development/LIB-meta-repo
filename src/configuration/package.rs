use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Package {
    pub dir: String,
    pub git: String,
}

impl Package {
    pub fn get_initial_config() -> Self {
        Self {
            dir: "".to_string(),
            git: "".to_string(),
        }
    }
}