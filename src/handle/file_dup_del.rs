use crate::command::file::DupDelArgs;
use crate::enumerate::file::FileHashType;
use crate::util;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use crate::error::Error;

pub fn handle(args: &DupDelArgs) -> Result<(), Error> {
    let mut hashes: HashMap<Vec<u8>, PathBuf> = HashMap::new();

    for entry in WalkDir::new(&args.dir) {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.file_name().unwrap().to_string_lossy().starts_with(".") {
            continue;
        }

        if path.is_file() {
            let hash = util::compute_file_hash(&FileHashType::SHA3, path).unwrap();

            if hashes.contains_key(&hash) {
                delete_file(hashes.get(&hash).unwrap(), &path, &hash);
            } else {
                hashes.insert(hash, path.to_path_buf());
            }
        }
    }

    println!("duplicates files clear finished");
    
    Ok(())
}

// delete file when sha3-256 and md5 is same
fn delete_file(exist: &Path, to_delete: &Path, hash_sha3: &Vec<u8>) {
    let hash_md5_exist = util::compute_file_hash(&FileHashType::MD5, to_delete).unwrap();
    let hash_md5_delete = util::compute_file_hash(&FileHashType::MD5, exist).unwrap();

    if hash_md5_exist == hash_md5_delete {
        match std::fs::remove_file(to_delete) {
            Ok(_) => println!("File {} deleted", to_delete.display()),
            Err(error) => println!("Error delete file {:?}:{:?}", to_delete, error),
        }
    } else {
        println!(
            "Files:\n\
    - {:?} (SHA3-256: {}, MD5: {})\n\
    - {:?} (SHA3-256: {}, MD5: {})\n\
    are the same in SHA3-256, but different in MD5.",
            exist,
            hex::encode(hash_sha3),
            hex::encode(hash_md5_exist),
            to_delete,
            hex::encode(hash_sha3),
            hex::encode(hash_md5_delete)
        );
    }
}
