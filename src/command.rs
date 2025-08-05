pub mod file;

use clap::{Subcommand};
use crate::command::file::FileCommand;
use crate::error::Error;

#[derive(Subcommand)]
pub enum Commands {
    #[command(subcommand, about="manage files")]
    File(FileCommand),
}

impl Commands {
    pub fn exec(&self) -> Result<(), Error> {
        match self {
            Commands::File(cmd) => cmd.exec(),
        }
    }
}