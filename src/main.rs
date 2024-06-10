mod configuration;
mod command;

fn main() {
    let matches = clap::Command::new("meta-repo")
        .subcommand(clap::Command::new("setup")
            .about("setup the meta-repo")
        )
        .get_matches();

    match matches.subcommand() {
        Some(("setup", _)) => {
            command::setup::setup();
        }
        _ => {
            eprintln!("No command provided");
        }
    }
}
