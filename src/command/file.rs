pub mod dup_del;
pub mod dup_list;
pub mod rename;

use crate::{
    command::file::{dup_del::DupDelArgs, dup_list::DupListArgs, rename::RenameArgs},
    error::Error,
};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum FileCommand {
    #[command(name = "dup-list", about = "list duplicate files")]
    DupList(DupListArgs),

    #[command(name = "dup-del", about = "delete duplicate files")]
    DupDel(DupDelArgs),

    #[command(
        name = "rename",
        about = "rename files using some rule, default is md5 value of file"
    )]
    Rename(RenameArgs), 
}

impl FileCommand {
    pub fn exec(&self) -> Result<(), Error> {
        match self {
            FileCommand::DupList(args) => args.exec(),
            FileCommand::DupDel(args) => args.exec(),
            FileCommand::Rename(args) => args.exec(),
        }
    }
}
