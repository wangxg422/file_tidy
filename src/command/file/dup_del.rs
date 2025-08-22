use crate::command::CommandExec;
use crate::enumerate::file::FileHashType;
use crate::error::Error;
use crate::util::hash::compute_file_hash;
use clap::Args;
use log::{error, info, warn};
use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use walkdir::WalkDir;

#[derive(Args)]
pub struct DupDelArgs {
    #[arg(long, help = "path of files", required = true)]
    pub dir: PathBuf,

    #[arg(
        short,
        long,
        help = "whether to recursively delete for duplicate files",
        required = false
    )]
    pub recursive: bool,
}

impl CommandExec for DupDelArgs {
    fn exec(&self) -> Result<(), Error> {
        let hashes: Arc<Mutex<HashMap<Vec<u8>, PathBuf>>> = Arc::new(Mutex::new(HashMap::new()));

        let mut walkdir = WalkDir::new(&self.dir);

        if !self.recursive {
            walkdir = walkdir.max_depth(1);
        }

        let entries: Vec<_> = walkdir
            .into_iter()
            .filter_entry(|e| {
                // 目录或文件名不是隐藏的才进入
                !e.file_name().to_string_lossy().starts_with('.')
            })
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file()) // 只要文件
            .map(|e| e.path().to_path_buf())
            .collect();

        entries.par_iter().for_each(|path| {
            match compute_file_hash(path, &FileHashType::SHA3_256) {
                Ok(hash) => {
                    if let Ok(mut map) = hashes.lock() {
                        if map.contains_key(&hash) {
                            let file = map.get(&hash).unwrap();
                            info!(
                                "File {:?}(SHA3-256: {}) exist, delete it",
                                file.display(),
                                hex::encode(hash),
                            );
                            match delete_file(file, &path) {
                                Ok(()) => info!("File {:?} successfully deleted", path.display()),
                                Err(err) => error!("File {:?} error: {}", path.display(), err),
                            }
                        } else {
                            map.insert(hash, path.to_path_buf());
                        }
                    } else {
                        error!("Failed to acquire lock when processing: {}", path.display());
                    }
                }
                Err(err) => {
                    error!("Failed to compute hash for {}: {}", path.display(), err);
                }
            }
        });

        info!("duplicates files delete finished");

        Ok(())
    }
}

// delete file if sha3-256 and md5 is same
fn delete_file(exist: &PathBuf, to_delete: &PathBuf) -> Result<(), Error> {
    let hash_md5_exist = compute_file_hash(to_delete, &FileHashType::MD5).unwrap();
    let hash_md5_delete = compute_file_hash(exist, &FileHashType::MD5).unwrap();

    if hash_md5_exist == hash_md5_delete {
        match std::fs::remove_file(to_delete) {
            Ok(_) => {
                info!("File {} deleted", to_delete.display());
                Ok(())
            }
            Err(err) => {
                error!("Error delete file {:?}:{:?}", to_delete, err);
                Err(Error::CustomError(format!(
                    "Failed to delete file {:?}: {}",
                    to_delete, err
                )))
            }
        }
    } else {
        warn!(
            "Files:\n\
    - {:?} (MD5: {})\n\
    - {:?} (MD5: {})\n\
    are the same in SHA3-256, but different in MD5.",
            exist,
            hex::encode(hash_md5_exist),
            to_delete,
            hex::encode(hash_md5_delete)
        );
        Err(Error::CustomError(format!(
            "File {} and {} are the same in SHA3-256, but different in MD5",
            exist.display(),
            to_delete.display()
        )))
    }
}
