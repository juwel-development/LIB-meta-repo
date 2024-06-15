use std::path::Path;
use std::{env, thread};

use git2::build::RepoBuilder;
use git2::{Cred, RemoteCallbacks};

use crate::command::install::install;
use crate::configuration::config::Config;

pub fn init() {
    println!("Starting initialization...");

    let config = Config::read_config();

    let mut handles = vec![];
    for app in config.apps {
        handles.push(thread::spawn(move || {
            clone_repo(&app.git, &app.dir);
        }));
    }
    for package in config.packages {
        handles.push(thread::spawn(move || {
            clone_repo(&package.git, &package.dir);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Initialization of completed.");

    install();
}

fn get_git_credentials(mut callbacks: RemoteCallbacks) -> RemoteCallbacks {
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        Cred::ssh_key(
            username_from_url.unwrap(),
            None,
            Path::new(&format!("{}/.ssh/id_rsa", env::var("HOME").unwrap())),
            None,
        )
    });

    callbacks
}

fn clone_repo(repo_url: &str, repo_dir: &str) {
    if repo_url.is_empty() || repo_dir.is_empty() {
        return;
    }

    if Path::new(repo_dir).exists() {
        println!("{} already exists, skipping cloning.", repo_dir);
        return;
    }

    let mut fo = git2::FetchOptions::new();
    let callbacks = RemoteCallbacks::new();
    fo.remote_callbacks(get_git_credentials(callbacks));

    let mut builder = RepoBuilder::new();
    builder.fetch_options(fo);

    println!("Cloning {} into {}", repo_dir, repo_url);
    builder.clone(repo_url, Path::new(repo_dir)).unwrap();
    println!("Cloned {} into {}", repo_url, repo_dir);
}
