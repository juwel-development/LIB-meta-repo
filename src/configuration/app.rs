use std::fs::File;
use std::io::BufReader;

use serde_json::Value;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
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

    pub fn get_name_from_package_json(&self) -> String {
        let package_json = format!("{}/package.json", self.dir);
        let file = File::open(package_json).unwrap();
        let reader = BufReader::new(file);
        let package_json: Value = serde_json::from_reader(reader).unwrap();
        let name = package_json["name"].as_str().unwrap();
        return name.to_string();
    }
}