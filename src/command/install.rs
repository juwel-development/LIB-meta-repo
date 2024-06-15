use std::sync::{Arc, Mutex};

use crate::configuration::config::Config;

pub fn install() {
    println!("Installing packages...");
    let config = Config::read_config();

    let mut handles = vec![];

    let linked_packages: Vec<String> = vec![];
    let lock = Arc::new(Mutex::new(linked_packages));

    for package in config.packages.clone() {
        let lock_internal = lock.clone();
        handles.push(std::thread::spawn(move || {
            let package_name = get_package_name(package.dir.clone());
            run_npm_install(&package_name, package.dir.clone());
            run_npm_link(&package_name, package.dir.as_str());
            lock_internal.lock().unwrap().push(package_name);
        }));
    }

    for app in config.apps.clone() {
        handles.push(std::thread::spawn(move || {
            let package_name = get_package_name(app.dir.clone());
            run_npm_install(&package_name, app.dir.clone());
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Installation completed.");
    let linked_packages = lock.lock().unwrap();
    println!("Linkable packages: {:?}", linked_packages);

    for package in config.packages.clone() {
        npm_link_dependencies(package.dir.as_str(), &linked_packages);
    }

    for app in config.apps {
        npm_link_dependencies(app.dir.as_str(), &linked_packages);
    }
}

fn run_npm_install(package_name: &String, dir: String) {
    let output = std::process::Command::new("npm")
        .arg("install")
        .current_dir(dir)
        .output()
        .expect("failed to execute npm install");

    println!(
        "{}: {}",
        package_name,
        String::from_utf8_lossy(&output.stdout)
    );
    println!(
        "{}: {}",
        package_name,
        String::from_utf8_lossy(&output.stderr)
    );
}

fn run_npm_link(package_name: &String, dir: &str) {
    let output = std::process::Command::new("npm")
        .arg("link")
        .current_dir(dir)
        .output()
        .expect("failed to execute npm link");

    println!(
        "{}: {}",
        package_name,
        String::from_utf8_lossy(&output.stdout)
    );
    println!(
        "{}: {}",
        package_name,
        String::from_utf8_lossy(&output.stderr)
    );
}

fn npm_link_dependencies(dir: &str, linked_packages: &[String]) {
    let package_json = std::fs::read_to_string(format!("{}/package.json", dir)).unwrap();
    let package_json_parsed: serde_json::Value = serde_json::from_str(&package_json).unwrap();

    let dependencies = package_json_parsed["dependencies"].as_object();
    if dependencies.is_none() {
        return;
    }

    let package_name = package_json_parsed["name"].as_str().unwrap();
    for (key, _) in dependencies.unwrap() {
        if linked_packages.contains(key) {
            println!("Linking dependency: {} -> {}", key, package_name);
            std::process::Command::new("npm")
                .arg("link")
                .arg(key)
                .current_dir(dir)
                .output()
                .expect("failed to execute npm link");
        }
    }
}

fn get_package_name(dir: String) -> String {
    let package_json = std::fs::read_to_string(format!("{}/package.json", dir)).unwrap();
    let package_json: serde_json::Value = serde_json::from_str(&package_json).unwrap();
    return package_json["name"].as_str().unwrap().to_string();
}
