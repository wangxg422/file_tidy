mod rename;
mod dup;

use clap::{Args, Subcommand};
use crate::command::file::dup::{DupArgs, DupCommand};
use crate::command::file::rename::{RenameArgs, RenameCommand};

#[derive(Subcommand)]
pub enum FileCommand {
    #[command(name = "dup-list", about = "list duplicate files")]
    Duplicates {
        cmd: DupCommand,
        args: DupArgs
    },

    #[command(name = "rename", about = "rename files using some rule, default is md5 value of file")]
    Rename {
        cmd: RenameCommand,
        args: RenameArgs
    },
}

impl FileCommand {
    pub fn exec(&self) {
        match self {
            FileCommand::Duplicates {cmd, args} => cmd.exec(),
            FileCommand::Rename {cmd, args} => cmd.exec()
        }
    }
}


