use std::env::current_dir;

#[cfg(not(target_os = "windows"))]
use std::os::unix::fs::symlink;
#[cfg(target_os = "windows")]
use std::os::windows::fs::{symlink_dir as symlink, symlink_file};
#[cfg(target_os = "windows")]
use std::path::Path;

use crate::configuration::config::Config;
use crate::configuration::package::Package;

pub fn start(app_name: String) {
    println!("Starting app: {}", app_name);

    let config = Config::read_config();

    let mut linked_packages: Vec<String> = vec![];

    // start build for all packages
    for package in config.packages.clone() {
        linked_packages.push(package.get_package_name());
        std::thread::spawn(move || {
            run_npm_link_dependency(&package);
            run_npm_start(&package.dir);
        });
    }

    // find the app in the config
    let app_config = config
        .apps
        .iter()
        .find(|app| app.get_package_name() == app_name);

    if app_config.is_none() {
        eprintln!("App not found in config");
        return;
    }

    let app_config = app_config.unwrap();

    println!("Linkable packages: {:?}", linked_packages);

    for package in config.packages.clone() {
        npm_link_dependencies(package.dir.as_str(), &linked_packages);
    }

    for app in config.apps.clone() {
        npm_link_dependencies(app.dir.as_str(), &linked_packages);
    }

    run_npm_start(&app_config.dir);
}

fn run_npm_start(dir: &String) {
    let _ = std::process::Command::new("npm")
        .arg("start")
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .current_dir(dir)
        .output();
}

fn run_npm_link_dependency(package: &Package) {
    let cache_dir = format!(
        "node_modules/.cache/meta-repo/{}",
        package.get_package_name()
    );
    let source_path = format!(
        "{}/{}/{}",
        current_dir().unwrap().display(),
        package.dir,
        package.output_dir
    );
    let destination_path = format!("{}/{}", cache_dir, package.output_dir);

    // Ensure source directory exists
    if !std::path::Path::new(&source_path).exists() {
        std::fs::create_dir_all(&source_path).expect("Failed to create source path");
    }
    // Create cache directory if it doesn't exist
    // clear cache directory if it exists
    if std::path::Path::new(&cache_dir).exists() {
        std::fs::remove_dir_all(&cache_dir).expect("Failed to remove cache directory");
    }

    if std::fs::create_dir_all(&cache_dir).is_err() {
        eprintln!("Failed to create cache directory: {}", cache_dir);
        return;
    }

    let _ = create_symlink(&source_path, &destination_path);

    // copy package.json
    let package_json = format!("{}/package.json", package.dir);
    let destination_package_json = format!("{}/package.json", cache_dir);
    std::fs::copy(package_json, &destination_package_json).expect("Failed to copy package.json");
    // remove scripts
    let package_json_content = std::fs::read_to_string(&destination_package_json).unwrap();
    let package_json_parsed: serde_json::Value =
        serde_json::from_str(&package_json_content).unwrap();
    let mut package_json_parsed = package_json_parsed.as_object().unwrap().clone();
    package_json_parsed.remove("scripts");
    let package_json_parsed = serde_json::to_string_pretty(&package_json_parsed).unwrap();
    std::fs::write(&destination_package_json, package_json_parsed)
        .expect("Failed to write package.json");

    // run npm link
    let _ = std::process::Command::new("npm")
        .arg("link")
        .current_dir(&cache_dir)
        .output()
        .expect("failed to execute npm link");
}

fn npm_link_dependencies(dir: &str, linked_packages: &[String]) {
    let package_json = std::fs::read_to_string(format!("{}/package.json", dir)).unwrap();
    let package_json_parsed: serde_json::Value = serde_json::from_str(&package_json).unwrap();

    let dependencies = package_json_parsed["dependencies"].as_object();
    if dependencies.is_none() {
        return;
    }

    let package_name = package_json_parsed["name"].as_str().unwrap();
    for (dependency_name, _) in dependencies.unwrap() {
        if linked_packages.contains(dependency_name) {
            println!(
                "Linking dependency: {} -> {}",
                dependency_name, package_name
            );
            std::process::Command::new("npm")
                .arg("link")
                .arg(dependency_name)
                .current_dir(dir)
                .stdin(std::process::Stdio::inherit())
                .stdout(std::process::Stdio::inherit());
        }
    }
}

fn create_symlink(source: &str, destination: &str) -> std::io::Result<()> {
    #[cfg(target_os = "windows")]
    {
        let source_path = Path::new(source);
        let destination_path = Path::new(destination);
        if source_path.is_dir() {
            symlink(source, destination)
        } else {
            symlink_file(source, destination)
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        symlink(source, destination)
    }
}
