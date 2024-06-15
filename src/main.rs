mod configuration;
mod command;

fn main() {
    let matches = clap::Command::new("meta-repo")
        .subcommand(clap::Command::new("setup")
            .about("setup the meta-repo")
        )
        .subcommand(clap::Command::new("init")
            .about("initialize the meta-repo")
        )
        .subcommand(clap::Command::new("install")
            .about("run npm install on all packages")
        )
        .subcommand(clap::Command::new("start-app")
            .about("start an app")
            .arg(clap::Arg::new("app")
                .required(true)
                .index(1)
            )
        )
        .get_matches();

    match matches.subcommand() {
        Some(("setup", _)) => {
            command::setup::setup();
        }
        Some(("init", _)) => {
            command::init::init();
        }
        Some(("install", _)) => {
            command::install::install();
        }
        Some(("start-app", arg_matcher)) => {
            let app = arg_matcher.get_one::<String>("app").unwrap().to_string();
            command::start::start(app);
        }
        _ => {
            eprintln!("No command provided");
        }
    }
}
