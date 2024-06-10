use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct App {
    pub dir: String,
    pub git: String,
}