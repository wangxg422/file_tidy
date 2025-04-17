mod command;
mod error;
mod util;
mod enumerate;
mod handle;

use crate::command::Commands;
use clap::Parser;

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
    let cli = Cli::parse();

    if cli.debug {
        println!("Debug mode enabled");
    }

    cli.command.exec();
}
