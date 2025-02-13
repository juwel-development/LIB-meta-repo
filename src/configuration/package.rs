use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Package {
    pub dir: String,
    pub git: String,
    pub output_dir: String,
}

impl Package {
    pub fn get_initial_config() -> Self {
        Self {
            dir: "".to_string(),
            git: "".to_string(),
            output_dir: "".to_string(),
        }
    }

    pub fn get_package_name(&self) -> String {
        let package_json = std::fs::read_to_string(format!("{}/package.json", self.dir)).unwrap();
        let package_json: serde_json::Value = serde_json::from_str(&package_json).unwrap();

        package_json["name"].as_str().unwrap().to_string()
    }
}
