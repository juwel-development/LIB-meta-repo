use std::fs::File;
use std::io::BufWriter;
use serde_json::to_string;
use crate::configuration::config::Config;

pub fn setup() {
    let file = File::create("meta-repo.config.json");
    let mut writer = BufWriter::new(file.unwrap());
    let config = Config::get_initial_config();

    to_string(&config).unwrap();

    serde_json::to_writer_pretty(&mut writer, &config).unwrap();
}