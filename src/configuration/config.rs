use std::fs::read_to_string;

use serde::{Deserialize, Serialize};

use crate::configuration::app::App;
use crate::configuration::package::Package;

pub static CONFIG_FILE: &str = "meta-repo.config.json";

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub apps: Vec<App>,
    pub packages: Vec<Package>,
}

impl Config {
    /**
     * initializes a configuration example for the meta-repo
     */
    pub fn get_initial_config() -> Self {
        Self {
            apps: vec![App::get_initial_config()],
            packages: vec![Package::get_initial_config()],
        }
    }

    pub fn read_config() -> Self {
        let file_content = read_to_string(CONFIG_FILE).unwrap();
        serde_json::from_str(&file_content).unwrap()
    }
}
