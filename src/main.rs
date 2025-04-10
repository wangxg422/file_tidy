mod command;

use crate::command::Commands;
use clap::Parser;

#[derive(Parser)]
#[command(name = "cli", version = "1.0", about = "A cli sample")]
#[command(help_template = "{bin} {version}

{about}

USAGE:

{usage}

 {all-args}")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, global = true)]
    verbose: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.verbose {
        println!("Verbose mode enabled");
    }

    cli.command.exec();
}
