use crate::enumerate::file::FileHashType;
use crate::{enumerate::sort::FileSort, error::Error};
use crate::handle;
use clap::{Args, Subcommand};
use std::path::PathBuf;

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

    #[arg(
        short,
        long,
        help = "where to save the duplicate files",
        required = false
    )]
    pub output: Option<String>,

    #[arg(
        short,
        long,
        help = "whether to recursively search for duplicate files",
        required = false
    )]
    pub recursive: bool,

    #[arg(
        short,
        long,
        help = "hash algorithm to compute the file digest: md5|sha1|sha256|sha3-224|sha3-256|sha3-384|sha3-512, default is sha3-256",
        required = false
    )]
    pub digest: FileHashType,
}

#[derive(Args)]
pub struct DupDelArgs {
    #[arg(short, long, help = "path of files", required = true)]
    pub dir: PathBuf,

    #[arg(
        short,
        long,
        help = "whether to recursively delete for duplicate files",
        required = false
    )]
    pub recursive: bool,
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

    #[arg(long = "lower", help = "lowercase file name", required = false)]
    pub lower: bool,

    #[arg(
        long = "upper-ext",
        help = "uppercase file extension",
        required = false
    )]
    pub upper_ext: bool,

    #[arg(
        long = "lower-ext",
        help = "lowercase file extension",
        required = false
    )]
    pub low_ext: bool,

    #[arg(
        long = "seq-len",
        help = "length of sequence, default 6",
        required = false,
        default_value = "6"
    )]
    pub seq_len: usize,

    #[command(flatten)]
    pub seq_sort: SeqSortArgs,

    #[arg(long = "prefix", help = "prefix of file name", required = false)]
    pub prefix: Option<String>,

    #[arg(long = "suffix", help = "suffix of file name", required = false)]
    pub suffix: Option<String>,

    #[arg(
        long = "ignore",
        help = "ignore file and dir, hidden file (which start with '.' is ignored default)",
        required = false
    )]
    pub ignore: Vec<String>,
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

    #[arg(
        long = "sha3-224",
        help = "rename file by sha3-224 value",
        required = false
    )]
    pub sha3_224: bool,

    #[arg(
        long = "sha3-256",
        help = "rename file by sha3-256 value",
        required = false
    )]
    pub sha3_256: bool,

    #[arg(
        long = "sha3-384",
        help = "rename file by sha3-384 value",
        required = false
    )]
    pub sha3_384: bool,

    #[arg(
        long = "sha3-512",
        help = "rename file by sha3-512 value",
        required = false
    )]
    pub sha3_512: bool,
}


#[derive(Args, Debug)]
pub struct SeqSortArgs {
    #[arg(
        long = "sort",
        help = "sort of file when rename file by sequence, one of name|size|time, default is name",
        required = false,
        default_value = "name"
    )]
    pub sort: FileSort,

    #[arg(
        long = "asc",
        help = "if `--sort` is setted, set asc to ",
        required = false,
        requires = "sort",
        group = "rename-seq-sort"
    )]
    pub asc: bool,

    #[arg(
        long = "desc",
        help = "if `--sort` is setted, set desc to ",
        required = false,
        requires = "sort",
        group = "rename-seq-sort"
    )]
    pub desc: bool,
}