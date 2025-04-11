pub mod file;

use clap::{Arg, Args, Subcommand};
use crate::command::file::FileCommand;

#[derive(Subcommand)]
pub enum Commands {
    #[command(subcommand, about="manage files")]
    File {
        cmd: FileCommand,
    },
}

pub struct FileCommandArgs {

}

impl CommandArgs for FileCommandArgs {}

impl CommandExec for Commands {
    fn exec(&self, _args: Option<&impl CommandArgs>) {
        match self {
            Commands::File { cmd} => cmd.exec(None),
        }
    }
}

pub trait CommandExec {
    fn exec(&self, args: Option<&impl CommandArgs>);
}

pub trait CommandArgs {
}

pub struct EmptyArgs {

}

impl CommandArgs for EmptyArgs {}