pub mod file;

use clap::{Arg, Args, Subcommand};
use crate::command::file::FileCommand;

#[derive(Subcommand)]
pub enum Commands {
    #[command(subcommand, about="manage files")]
    File(FileCommand),
}

impl Commands {
    pub fn exec(&self) {
        match self {
            Commands::File(cmd) => cmd.exec(),
        }
    }
}