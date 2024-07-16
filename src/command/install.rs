use crate::configuration::config::Config;

pub fn install() {
    println!("Installing packages...");
    let config = Config::read_config();

    let mut handles = vec![];

    for package in config.packages.clone() {
        handles.push(std::thread::spawn(move || {
            let package_name = package.get_package_name();
            run_npm_install(&package_name, package.dir.clone());
        }));
    }

    for app in config.apps.clone() {
        handles.push(std::thread::spawn(move || {
            let package_name = app.get_package_name();
            run_npm_install(&package_name, app.dir.clone());
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Installation completed.");
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
