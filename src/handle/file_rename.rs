use crate::command::file::RenameArgs;
use crate::enumerate::file::FileHashType::{MD5, SHA1, SHA3, SHA256};
use crate::util::compute_file_hash;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub fn handle(args: &RenameArgs) {
    if !args.dir.exists() {
        println!("path does not exist: {}", args.dir.display());
        return;
    }

    let naming_by = if args.naming_rule.md5 {
        println!("rename file by md5");
        MD5
    } else if args.naming_rule.sha1 {
        println!("rename file by sha1");
        SHA1
    } else if args.naming_rule.sha256 {
        println!("rename file by sha256");
        SHA256
    } else if args.naming_rule.sha3 {
        println!("rename file by sha3-256");
        SHA3
    } else if args.naming_rule.sequence {
        println!("rename file by sequence");
        return rename_by_seq(args);
    } else {
        panic!("unknown naming rule");
    };

    for entry in WalkDir::new(&args.dir) {
        let entry = entry.unwrap();
        let path = entry.path();

        if is_ignore(path, &args.ignore) {
            println!("file {} is ignored", path.display());
            continue;
        }

        if path.is_file() {
            let hash = compute_file_hash(&naming_by, path).unwrap();
            let new_name = if args.upper {
                hex::encode_upper(hash)
            } else {
                hex::encode(hash)
            };

            let new_path = match path.extension() {
                Some(ext) => {
                    let mut ext_name = ext.to_str().unwrap().to_string();
                    if args.upper_ext {
                        ext_name = ext_name.to_ascii_uppercase();
                    }
                    if args.low_ext {
                        ext_name = ext_name.to_ascii_lowercase();
                    }
                    path.with_file_name(format!("{}.{}", &new_name, ext_name))
                }
                None => path.with_file_name(&new_name),
            };

            println!("Renamed {:?} to {:?}", path, new_path);
            fs::rename(&path, &new_path).unwrap_or_else(|err| {
                panic!(
                    "Failed to rename file {:?} to {:?}: {}",
                    path, new_path, err
                );
            });
        }
    }

    println!("file renamed finished");
}

fn rename_by_seq(args: &RenameArgs) {
    let mut seq = 1;
    for entry in WalkDir::new(&args.dir) {
        let entry = entry.unwrap();
        let path = entry.path();

        if is_ignore(path, &args.ignore) {
            println!("file {} is ignored", path.display());
            continue;
        }

        if path.is_dir() {
            seq = 1;
        }

        if path.is_file() {
            let new_name = format!("{:0width$}", seq, width = args.seq_len);
            seq = seq + 1;

            let new_path = match path.extension() {
                Some(ext) => {
                    let mut ext_name = ext.to_str().unwrap().to_string();
                    if args.upper_ext {
                        ext_name = ext_name.to_ascii_uppercase();
                    }
                    if args.low_ext {
                        ext_name = ext_name.to_ascii_lowercase();
                    }
                    path.with_file_name(format!("{}.{}", &new_name, ext_name))
                }
                None => path.with_file_name(&new_name),
            };

            println!("Renamed {:?} to {:?}", path, new_path);
            fs::rename(&path, &new_path).unwrap_or_else(|err| {
                panic!(
                    "Failed to rename file {:?} to {:?}: {}",
                    path, new_path, err
                );
            });
        }
    }
}

fn is_ignore(path: &Path, _ignore: &Vec<String>) -> bool {
    let name = path.file_name().unwrap().to_string_lossy();
    if path.is_file() {
        if name.starts_with(".") {
            return true
        }
    }

    if path.is_dir() {
        if name.starts_with(".") {
            return true
        }
    }

    false
}
