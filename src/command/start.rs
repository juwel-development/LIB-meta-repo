use crate::configuration::config::Config;

pub fn start(app_name: String) {
    println!("Starting app: {}", app_name);

    // find the app in the config
    let config = Config::read_config();
    let app_config = config.apps.iter().find(|app| {
        let name = app.get_name_from_package_json();

        return name == app_name;
    });

    if app_config.is_none() {
        eprintln!("App not found in config");
        return;
    }

    let app_config = app_config.unwrap();
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