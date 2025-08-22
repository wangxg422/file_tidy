pub mod file;

use clap::{Subcommand};
use crate::command::file::FileCommand;
use crate::error::Error;

pub trait CommandExec {
    fn exec(&self) -> Result<(), Error>;
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(subcommand, about="manage files")]
    File(FileCommand),
}

impl CommandExec for Commands {
    fn exec(&self) -> Result<(), Error> {
        match self {
            Commands::File(cmd) => cmd.exec(),
        }
    }
}