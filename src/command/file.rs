mod rename;
mod dup;

use clap::{Args, Subcommand};
use std::path::PathBuf;
use crate::command::file::dup::DuplicatesArgs;
use crate::command::file::rename::{NamingRule, RenameArgs};

#[derive(Subcommand)]
pub enum FileCommand {
    #[command(about = "list duplicate files")]
    Duplicates(DuplicatesArgs),

    #[command(about = "rename files using some rule, default is md5 value of file")]
    Rename(RenameArgs),
}

impl FileCommand {
    pub fn exec(&self) {
        match self {
            FileCommand::Duplicates(args) => {
                println!("Duplicates dir: {}", args.dir);
                println!("Duplicates output: {}", args.output);
            }
            FileCommand::Rename(args) => {
                if !args.dir.exists() {
                    println!("path does not exist: {}", args.dir.display());
                    return;
                }

                if args.naming_rule.sha1 {
                    rename::rename_files(&args.dir, NamingRule::SHA1, true && args.lower, args.lower_ext);
                } else if args.naming_rule.sha256 {
                    "SHA-256"
                } else if args.naming_rule.sha3 {
                    "SHA3"
                } else if args.naming_rule.md5 {
                    "MD5"
                } else if args.naming_rule.sequence{
                    "Sequential"
                }
            }
        }
    }
}


