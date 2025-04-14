use std::fs;
use walkdir::WalkDir;
use crate::command::file::RenameArgs;
use crate::enumerate::file::FileHashType::{MD5, SHA1, SHA256, SHA3};
use crate::util::compute_file_hash;

pub fn handle(args: &RenameArgs) {
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