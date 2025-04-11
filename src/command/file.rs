mod rename;
mod dup;

use clap::{Args, Subcommand};
use crate::command::{CommandArgs, CommandExec, FileCommandArgs};
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

impl CommandExec for FileCommand {
    fn exec(&self, _args: &FileCommandArgs) {
        match self {
            FileCommand::Duplicates {cmd, args} => cmd.exec(args),
            FileCommand::Rename {cmd, args} => cmd.exec(args)
        }
    }
}


