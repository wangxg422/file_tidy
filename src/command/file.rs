use std::path::PathBuf;
use clap::{Args, Subcommand};
use crate::handle::{file_dup, file_rename};

#[derive(Subcommand)]
pub enum FileCommand {
    #[command(name = "dup-list", about = "list duplicate files")]
    Duplicates(DupArgs),

    #[command(name = "rename", about = "rename files using some rule, default is md5 value of file")]
    Rename(RenameArgs),
}

impl FileCommand {
    pub fn exec(&self) {
        match self {
            FileCommand::Duplicates(args) => file_dup::exec(&args),
            FileCommand::Rename(args) => file_rename::exec(&args)
        }
    }
}

#[derive(Args)]
pub struct DupArgs {
    #[arg(short, long, help = "path of files")]
    pub dir: String,

    #[arg(short, long, help = "where to save the duplicate files, default is ")]
    pub output: String,
}

#[derive(Args)]
pub struct RenameArgs {
    #[arg(short, long, help = "path of files", required = true)]
    pub dir: PathBuf,

    /// Naming rule to apply (choose one)
    #[command(flatten)]
    pub naming_rule: NamingRuleArgs,

    #[arg(long = "lower", help = "lowercase file name", required = false)]
    pub lower: bool,

    #[arg(
        long = "lower-ext",
        help = "lowercase file extension",
        required = false
    )]
    pub lower_ext: bool,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
struct NamingRuleArgs {
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


