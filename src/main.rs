use std::fs::File;
use std::io::BufWriter;

use serde_json::to_string;

use configuration::app::App;

use crate::configuration::config::Config;
use crate::configuration::package::Package;

mod configuration;

fn main() {
    let matches = clap::Command::new("meta-repo")
        .subcommand(clap::Command::new("init")
            .about("Initialize a new project")
        )
        .get_matches();

    match matches.subcommand() {
        Some(("init", _)) => {
            let file = File::create("meta-repo.config.json");
            let mut writer = BufWriter::new(file.unwrap());
            let config = Config {
                apps: vec![
                    App {
                        dir: "".to_string(),
                        git: "".to_string(),
                    }],
                packages: vec![Package {
                    dir: "".to_string(),
                    git: "".to_string(),
                }],
            };

            to_string(&config).unwrap();

            serde_json::to_writer_pretty(&mut writer, &config).unwrap();
        }
        _ => {
            eprintln!("No command provided");
        }
    }
}
