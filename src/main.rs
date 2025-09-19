use clap::Parser;
mod init;
mod sync;
mod config;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    init: bool,
    #[arg(short, long)]
    sync: bool,
}

fn main() {
    let args = Args::parse();
    if args.init && args.sync {
        eprintln!("Cannot use --init and --sync together");
        std::process::exit(1);
    }
    if args.init {
        return init::init();
    }
    if args.sync {
        sync::sync(args);
    }
}
