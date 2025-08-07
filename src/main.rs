mod command;
mod error;
mod util;
mod enumerate;
mod handle;

use crate::command::Commands;
use clap::Parser;
use log::{debug, error, info};

#[derive(Parser)]
#[command(name = "file-tidy", version = "v0.1.0", about = "A cli sample")]
#[command(help_template = "{bin} {version}

{about}

USAGE:

{usage}

 {all-args}")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long)]
    debug: bool,

    #[arg(short, long, global = true)]
    verbose: bool,
}

fn main() {
    log4rs::init_file("src/log4rs.yaml", Default::default()).unwrap();

    let cli = Cli::parse();

    if cli.debug {
        debug!("Debug mode enabled")
    }

    match cli.command.exec() {
        Ok(()) => {
            info!("executed finished");
        },
        Err(err) => error!("Error: {}", err),
    }
}
