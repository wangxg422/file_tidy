use std::collections::BTreeMap;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use clap::{Args, Subcommand};
use walkdir::WalkDir;
use crate::enumerate::file::NamingRule;
use crate::enumerate::file::NamingRule::{MD5, SEQUENCE, SHA1, SHA256, SHA3};
use crate::util;
use crate::util::compute_file_hash;

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
            FileCommand::Duplicates(args) => {
                let mut hashes: BTreeMap<Vec<u8>, Vec<PathBuf>> = BTreeMap::new();

                for entry in WalkDir::new(&args.dir) {
                    let entry = entry.unwrap();
                    let path = entry.path();

                    if path.is_file() {
                        let hash = util::compute_file_hash(&NamingRule::MD5, path).unwrap();

                        hashes.entry(hash)
                            .or_insert_with(Vec::new)
                            .push(path.to_path_buf());
                    }
                }

                if args.output.is_empty() {
                    save_duplicates_to_file(&args.dir, &mut hashes);
                } else {
                    print_duplicates(&args.dir, &mut hashes);
                }
            },
            FileCommand::Rename(args) => {
                if !args.dir.exists() {
                    println!("path does not exist: {}", args.dir.display());
                    return;
                }

                let mut naming_by = MD5;
                if args.naming_rule.md5 {
                    naming_by = MD5
                } else if args.naming_rule.sha1 {
                    naming_by = SHA1;
                } else if args.naming_rule.sha256 {
                    naming_by = SHA256
                } else if args.naming_rule.sha3 {
                    naming_by = SHA3
                } else if args.naming_rule.sequence {
                    naming_by = SEQUENCE
                }

                for entry in WalkDir::new(&args.dir) {
                    let entry = entry.unwrap();
                    let path = entry.path();

                    if path.is_file() {
                        let new_name = hex::encode(compute_file_hash(&naming_by, path).unwrap());

                        let new_path = match path.extension() {
                            Some(ext) => path.with_file_name(format!("{}.{}", &new_name, ext.to_str().unwrap())),
                            None => path.with_file_name(&new_name)
                        };

                        println!("Renamed {:?} to {:?}", path, new_path);
                        fs::rename(&path, &new_path).unwrap_or_else(|err| {
                            panic!("Failed to rename file {:?} to {:?}: {}", path, new_path, err);
                        });
                    }
                }
            }
        }
    }
}

#[derive(Args)]
pub struct DupArgs {
    #[arg(short, long, help = "path of files")]
    pub dir: PathBuf,

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

    #[arg(long = "upper", help = "uppercase file name", required = false)]
    pub upper: bool,

    #[arg(
        long = "lower-ext",
        help = "lowercase file extension",
        required = false
    )]
    pub lower_ext: bool,
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

fn print_duplicates(path: &PathBuf, hashes: &mut BTreeMap<Vec<u8>, Vec<PathBuf>>) {
    println!("Duplicate files in {}:\n\n", path.as_os_str().to_str().unwrap());

    for files in hashes.values_mut() {
        files.sort();
    }

    for (hash, paths) in hashes {
        if paths.len() > 1 {
            println!("Duplicate files sha3: {}\n", hex::encode(hash));
            paths.sort();
            for path in paths {
                println!("    - {}\n", path.display());
            }
            println!("{}", "\n");
        }
    }
}

fn save_duplicates_to_file(path: &PathBuf, hashes: &mut BTreeMap<Vec<u8>, Vec<PathBuf>>) {

    let detail= "__duplicates.txt";

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(detail).unwrap();

    file.write_all(format!("Duplicate files in {}:\n\n", path.as_os_str().to_str().unwrap()).as_bytes()).expect("write to file failed");

    for files in hashes.values_mut() {
        files.sort();
    }

    for (hash, paths) in hashes {
        if paths.len() > 1 {
            file.write_all(format!("Duplicate files sha3: {}\n", hex::encode(hash)).as_bytes()).expect("write to file failed");
            paths.sort();
            for path in paths {
                file.write_all(format!("    - {}\n", path.display()).as_bytes()).expect("write to file failed");
            }
            file.write_all(format!("{}", "\n").as_bytes()).expect("write to file failed");
        }
    }

    println!("Duplicate files check finished, please confirm at: {}\n", detail);
}

