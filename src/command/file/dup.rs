use clap::Args;
use crate::command::{CommandArgs, CommandExec};

pub struct DupCommand {}

impl CommandExec for DupCommand {
    fn exec(&self, _args: &DuplicatesArgs) {

    }
}

#[derive(Args)]
pub struct DuplicatesArgs {
    #[arg(short, long, help = "path of files")]
    pub dir: String,

    #[arg(short, long, help = "where to save the duplicate files, default is ")]
    pub output: String,
}

impl CommandArgs for DuplicatesArgs {}