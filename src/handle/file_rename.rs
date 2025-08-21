use crate::command::file::RenameArgs;
use crate::enumerate::file::FileHashType::{
    MD5, SHA1, SHA3_224, SHA3_256, SHA3_384, SHA3_512, SHA256,
};
use crate::enumerate::sort::FileSort;
use crate::error::Error;
use crate::util::hash::compute_file_hash;
use log::{debug, error, info};
use rayon::prelude::*;
use std::fs;
use walkdir::WalkDir;

pub fn handle(args: &RenameArgs) -> Result<(), Error> {
    if !args.dir.exists() {
        error!("path does not exist: {}", args.dir.display());
        return Err(Error::CustomError(format!(
            "path does not exist: {}",
            args.dir.display()
        )));
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
    } else if args.naming_rule.sha3_224 {
        info!("rename file by sha3-224");
        SHA3_224
    } else if args.naming_rule.sha3_256 {
        info!("rename file by sha3-256");
        SHA3_256
    } else if args.naming_rule.sha3_384 {
        info!("rename file by sha3-384");
        SHA3_384
    } else if args.naming_rule.sha3_512 {
        info!("rename file by sha3-512");
        SHA3_512
    } else if args.naming_rule.sequence {
        info!("rename file by sequence");
        return rename_by_seq(args);
    } else {
        error!("unknown naming rule");
        return Err(Error::CustomError("unknown naming rule".to_string()));
    };

    let entries: Vec<_> = WalkDir::new(&args.dir)
        .max_depth(1)
        .into_iter()
        .filter_entry(|e| {
            // 目录或文件名不是隐藏的才进入
            !e.file_name().to_string_lossy().starts_with('.')
        })
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file()) // 只要文件
        .map(|e| e.path().to_path_buf())
        .collect();

    entries.par_iter().for_each(|file| {
        if let Ok(hash) = compute_file_hash(file, &naming_by) {
            let new_name = if args.upper {
                hex::encode_upper(hash)
            } else {
                hex::encode(hash)
            };

            let new_path = match file.extension() {
                Some(ext) => {
                    let mut ext_name = ext.to_str().unwrap().to_string();
                    if args.upper_ext {
                        ext_name = ext_name.to_ascii_uppercase();
                    }
                    if args.low_ext {
                        ext_name = ext_name.to_ascii_lowercase();
                    }

                    let prefix = &args.prefix.as_deref().unwrap_or("");
                    let suffix = &args.suffix.as_deref().unwrap_or("");

                    file.with_file_name(format!("{}{}{}.{}", prefix, &new_name, suffix, ext_name))
                }
                None => file.with_file_name(&new_name),
            };

            info!("renamed {:?} to {:?}", file, new_path);
            fs::rename(&file, &new_path).unwrap_or_else(|err| {
                error!(
                    "Failed to rename file {:?} to {:?}: {}",
                    file, new_path, err
                );
            });
        } else {
            error!("Failed to compute hash for file {:?}", file);
        }
    });

    info!("file renamed finished");
    Ok(())
}

fn rename_by_seq(args: &RenameArgs) -> Result<(), Error> {
    let mut entries: Vec<_> = WalkDir::new(&args.dir)
        .max_depth(1)
        .into_iter()
        .filter_entry(|e| {
            // 目录或文件名不是隐藏的才进入
            !e.file_name().to_string_lossy().starts_with('.')
        })
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file()) // 只要文件
        .map(|e| e.path().to_path_buf())
        .collect();

    match args.seq_sort.sort {
        FileSort::Name => {
            if args.seq_sort.desc {
                debug!("sort by file name, desc");
                entries.sort_by(|a, b| b.cmp(a))
            } else {
                debug!("sort by file name, asc");
                entries.sort_by(|a, b| a.cmp(b))
            }
        },
        FileSort::Size => {
            if args.seq_sort.desc {
                debug!("sort by file size, desc");
                entries.sort_by(|a, b| {
                    let a = a.metadata().unwrap().len();
                    let b = b.metadata().unwrap().len();
                    b.cmp(&a)
                })
            } else {
                debug!("sort by file size, asc");
                entries.sort_by(|a, b| {
                    let a = a.metadata().unwrap().len();
                    let b = b.metadata().unwrap().len();
                    a.cmp(&b)
                })
            }
        },
        FileSort::Time => {
            if args.seq_sort.desc {
                debug!("sort by last update time, desc");
                entries.sort_by(|a, b| {
                    let a = a.metadata().unwrap().modified().unwrap();
                    let b = b.metadata().unwrap().modified().unwrap();
                    b.cmp(&a)
                })
            } else {
                debug!("sort by last update time, desc");
                entries.sort_by(|a, b| {
                    let a = a.metadata().unwrap().modified().unwrap();
                    let b = b.metadata().unwrap().modified().unwrap();
                    a.cmp(&b)
                })
            }
        },
    }

    // 使用 enumerate 分配序号，避免共享可变 state
    entries.par_iter().enumerate().for_each(|(i, file)| {
        let new_name = format!("{:0width$}", i + 1, width = args.seq_len);

        let new_path = match file.extension().and_then(|s| s.to_str()) {
            Some(ext) if !ext.is_empty() => {
                let ext_name = if args.upper_ext {
                    ext.to_ascii_uppercase()
                } else if args.low_ext {
                    ext.to_ascii_lowercase()
                } else {
                    ext.to_string()
                };

                let prefix = &args.prefix.as_deref().unwrap_or("");
                let suffix = &args.suffix.as_deref().unwrap_or("");

                file.with_file_name(format!("{}{}{}.{}", prefix, new_name, suffix, ext_name))
            }
            _ => file.with_file_name(&new_name),
        };

        info!("renamed {:?} to {:?}", file, new_path);
        if let Err(err) = fs::rename(file, &new_path) {
            error!(
                "Failed to rename file {:?} to {:?}: {}",
                file, new_path, err
            );
        }
    });

    Ok(())
}
