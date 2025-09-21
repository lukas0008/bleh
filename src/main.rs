use clap::{Arg, ArgAction, Command, Parser};
mod init;
mod sync;
mod config;

fn main() {
    let cmd = Command::new("bleh")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("init")
                .short_flag('I')
                .long_flag("init")
        )
        .subcommand(
            Command::new("sync")
                .short_flag('S')
                .long_flag("sync")
                .arg(
                    Arg::new("package")
                        .help("packages")
                        .action(ArgAction::Set)
                        .num_args(1..)
                )
        )
        .get_matches();

    match cmd.subcommand() {
        Some(("init", init_matches)) => {
            init::init();
        }

        Some(("sync", sync_matches)) => {
            let packages = sync_matches
                .get_many::<String>("package")
                .expect("is present")
                .map(|s| s.as_str())
                .collect();

            sync::sync(&packages);
        }

        _ => unreachable!()
    }
}
