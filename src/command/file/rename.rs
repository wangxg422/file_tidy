use clap::Args;
use std::path::PathBuf;

pub struct RenameCommand {}

impl RenameCommand {
    fn exec(&self) {
        // if !args.dir.exists() {
        //     println!("path does not exist: {}", args.dir.display());
        //     return;
        // }
        //
        // if args.naming_rule.sha1 {
        //
        // } else if args.naming_rule.sha256 {
        //
        // } else if args.naming_rule.sha3 {
        //
        // } else if args.naming_rule.md5 {
        //
        // } else if args.naming_rule.sequence{
        //
        // }
    }
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

pub enum NamingRule {
    SHA1,
    SHA256,
    SHA3,
    SEQUENCE,
}
