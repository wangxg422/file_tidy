use std::path::PathBuf;
use clap::{Args, Subcommand};
use crate::handle;

#[derive(Subcommand)]
pub enum FileCommand {
    #[command(name = "dup-list", about = "list duplicate files")]
    DupList(DupListArgs),

    #[command(name = "dup-del", about = "delete duplicate files")]
    DupDel(DupDelArgs),

    #[command(name = "rename", about = "rename files using some rule, default is md5 value of file")]
    Rename(RenameArgs),
}

impl FileCommand {
    pub fn exec(&self) {
        match self {
            FileCommand::DupList(args) => handle::file_dup_list::handle(args),
            FileCommand::DupDel(args) => handle::file_dup_del::handle(args),
            FileCommand::Rename(args) => handle::file_rename::handle(args),
        }
    }
}

#[derive(Args)]
pub struct DupListArgs {
    #[arg(short, long, help = "path of files", required = true)]
    pub dir: PathBuf,

    #[arg(short, long, help = "where to save the duplicate files", required = false)]
    pub output: Option<String>,
}

#[derive(Args)]
pub struct DupDelArgs {
    #[arg(short, long, help = "path of files", required = true)]
    pub dir: PathBuf,
}

#[derive(Args)]
pub struct RenameArgs {
    #[arg(short, long, help = "path of files", required = true)]
    pub dir: PathBuf,

    /// Naming rule to apply (choose one)
    #[command(flatten)]
    pub naming_rule: NamingRuleArgs,

    #[arg(long = "upper", help = "uppercase file name", required = false)]
    pub upper: bool,

    #[arg(
        long = "upper-ext",
        help = "uppercase file extension",
        required = false
    )]
    pub upper_ext: bool,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
pub struct NamingRuleArgs {
    #[arg(long = "seq", help = "rename file by sequence", required = false)]
    pub sequence: bool,

    #[arg(long = "md5", help = "rename file by md5 value", required = false)]
    pub md5: bool,

    #[arg(long = "sha1", help = "rename file by sha1 value", required = false)]
    pub sha1: bool,

    #[arg(
        long = "sha256",
        help = "rename file by sha256 value",
        required = false
    )]
    pub sha256: bool,

    #[arg(long = "sha3", help = "rename file by sha3 value", required = false)]
    pub sha3: bool,
}
