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
        .get_matches();

    match matches.subcommand() {
        Some(("setup", _)) => {
            command::setup::setup();
        }
        Some(("init", _)) => {
            command::init::init();
        }
        _ => {
            eprintln!("No command provided");
        }
    }
}
