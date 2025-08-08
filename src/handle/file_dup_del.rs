use crate::command::file::DupDelArgs;
use crate::enumerate::file::FileHashType;
use crate::error::Error;
use crate::util::hash::compute_file_hash;
use log::{error, info, warn};
use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use walkdir::WalkDir;

pub fn handle(args: &DupDelArgs) -> Result<(), Error> {
    let hashes: Arc<Mutex<HashMap<Vec<u8>, PathBuf>>> = Arc::new(Mutex::new(HashMap::new()));

    let entries: Vec<_> = WalkDir::new(&args.dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && !e.file_name().to_string_lossy().starts_with("."))
        .map(|e| e.path().to_path_buf())
        .collect();

    entries
        .par_iter()
        .for_each(|path| match compute_file_hash(&FileHashType::SHA3, path) {
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
        });

    info!("duplicates files delete finished");

    Ok(())
}

// delete file if sha3-256 and md5 is same
fn delete_file(exist: &Path, to_delete: &Path) -> Result<(), Error> {
    let hash_md5_exist = compute_file_hash(&FileHashType::MD5, to_delete).unwrap();
    let hash_md5_delete = compute_file_hash(&FileHashType::MD5, exist).unwrap();

    if hash_md5_exist == hash_md5_delete {
        match std::fs::remove_file(to_delete) {
            Ok(_) => {
                info!("File {} deleted", to_delete.display());
                Ok(())
            }
            Err(err) => {
                error!("Error delete file {:?}:{:?}", to_delete, err);
                Err(Error::CustomError(format!("Failed to delete file {:?}: {}", to_delete, err)))
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
        Err(Error::CustomError(format!("File {} and {} are the same in SHA3-256, but different in MD5", exist.display(), to_delete.display())))
    }
}
