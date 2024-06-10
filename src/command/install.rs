use std::sync::{Arc, Mutex};

use crate::configuration::config::Config;

pub fn install() {
    println!("Installing packages...");
    let config = Config::read_config();

    let mut handles = vec![];

    let linked_packages: Vec<String> = vec![];
    let lock = Arc::new(Mutex::new(linked_packages));

    for package in config.packages {
        let lock_internal = lock.clone();
        handles.push(std::thread::spawn(move || {
            run_npm_install(package.dir.clone());
            let package_name = run_npm_link(package.dir.as_str());
            lock_internal.lock().unwrap().push(package_name);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let linked_packages = lock.lock().unwrap();
    println!("Linked packages: {:?}", linked_packages);
}

fn run_npm_install(dir: String) {
    let output = std::process::Command::new("npm")
        .arg("install")
        .current_dir(dir)
        .output()
        .expect("failed to execute npm install");

    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("{}", String::from_utf8_lossy(&output.stderr));
}

fn run_npm_link(dir: &str) -> String {
    let output = std::process::Command::new("npm")
        .arg("link")
        .current_dir(dir)
        .output()
        .expect("failed to execute npm link");

    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("{}", String::from_utf8_lossy(&output.stderr));

    //read package.json
    let package_json = std::fs::read_to_string(format!("{}/package.json", dir)).unwrap();
    let package_json: serde_json::Value = serde_json::from_str(&package_json).unwrap();
    let package_name = package_json["name"].as_str().unwrap();

    return package_name.to_string();
}