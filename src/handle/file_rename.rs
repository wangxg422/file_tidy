use crate::command::file::RenameArgs;
use crate::enumerate::file::FileHashType::{MD5, SHA1, SHA3_256, SHA256};
use crate::util::hash::compute_file_hash;
use std::fs;
use std::path::Path;
use log::{error, info};
use walkdir::WalkDir;
use crate::error::Error;
use rayon::prelude::*;

pub fn handle(args: &RenameArgs) -> Result<(), Error> {
    if !args.dir.exists() {
        error!("path does not exist: {}", args.dir.display());
        return Err(Error::CustomError(format!("path does not exist: {}", args.dir.display())));
    }

    let naming_by = if args.naming_rule.md5 {
        info!("rename file by md5");
        MD5
    } else if args.naming_rule.sha1 {
        info!("rename file by sha1");
        SHA1
    } else if args.naming_rule.sha256 {
        info!("rename file by sha256");
        SHA256
    } else if args.naming_rule.sha3 {
        info!("rename file by sha3-256");
        SHA3_256
    } else if args.naming_rule.sequence {
        info!("rename file by sequence");
        return rename_by_seq(args);
    } else {
        error!("unknown naming rule");
        return Err(Error::CustomError("unknown naming rule".to_string()));
    };

    let entries: Vec<_> = WalkDir::new(&args.dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && !e.file_name().to_string_lossy().starts_with("."))
        .map(|e| e.path().to_path_buf())
        .collect();

    entries.par_iter().for_each(|file | {

    });

    for entry in WalkDir::new(&args.dir) {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            let hash = compute_file_hash(path, &naming_by).unwrap();
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

            info!("Renamed {:?} to {:?}", path, new_path);
            fs::rename(&path, &new_path).unwrap_or_else(|err| {
                error!(
                    "Failed to rename file {:?} to {:?}: {}",
                    path, new_path, err
                );
            });
        }
    }

    info!("file renamed finished");
    Ok(())
}

fn rename_by_seq(args: &RenameArgs) -> Result<(), Error> {
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

            info!("Renamed {:?} to {:?}", path, new_path);
            fs::rename(&path, &new_path).unwrap_or_else(|err| {
                error!(
                    "Failed to rename file {:?} to {:?}: {}",
                    path, new_path, err
                );
            });
        }
    }

    Ok(())
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
