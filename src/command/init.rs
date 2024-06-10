use std::{env, fs};
use std::path::Path;

use git2::{Cred, RemoteCallbacks};

use crate::configuration::config::Config;
use crate::configuration::config::CONFIG_FILE;

pub fn init() {
    println!("Starting initialization of the meta repo...");

    let file_content = fs::read_to_string(CONFIG_FILE).unwrap();
    let config: Config = serde_json::from_str(&file_content).unwrap();

    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        Cred::ssh_key(
            username_from_url.unwrap(),
            None,
            Path::new(&format!("{}/.ssh/id_rsa", env::var("HOME").unwrap())),
            None,
        )
    });

    // Prepare fetch options.
    let mut fo = git2::FetchOptions::new();
    fo.remote_callbacks(callbacks);

    // Prepare builder.
    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fo);

    for app in config.apps {
        if &app.dir == "" || &app.git == "" {
            continue;
        }

        if Path::new(&app.dir).exists() {
            println!("{} already exists, skipping cloning.", app.dir);
            continue;
        }

        println!("Cloning {} into {}", app.git, app.dir);
        let repo = builder.clone(&app.git, Path::new(&app.dir)).unwrap();
        println!("Cloned {} into {}", repo.workdir().unwrap().display(), app.dir);
    }

    for package in config.packages {
        if &package.dir == "" || &package.git == "" {
            continue;
        }

        if Path::new(&package.dir).exists() {
            println!("{} already exists, skipping cloning.", package.dir);
            continue;
        }
        // Clone the repository.
        println!("Cloning {} into {}", package.git, package.dir);
        let repo = builder.clone(&package.git, Path::new(&package.dir)).unwrap();
        println!("Cloned {} into {}", repo.workdir().unwrap().display(), package.dir);
    }

    println!("Initialization of the meta repo completed.");
}