use crate::command::file::DupListArgs;
use crate::enumerate::file::FileHashType;
use crate::util::hash::compute_file_hash;
use std::collections::BTreeMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use log::{error, info};
use walkdir::WalkDir;
use rayon::prelude::*;
use crate::error::Error;

pub fn handle(args: &DupListArgs) -> Result<(), Error> {
    let hashes: Arc<Mutex<BTreeMap<Vec<u8>, Vec<PathBuf>>>> = Arc::new(Mutex::new(BTreeMap::new()));


    let entries: Vec<_> = WalkDir::new(&args.dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && !e.file_name().to_string_lossy().starts_with("."))
        .map(|e| e.path().to_path_buf())
        .collect();

    entries.par_iter().for_each(|path| {
        match compute_file_hash(&FileHashType::SHA3, path) {
            Ok(hash) => {
                // 尝试加锁
                if let Ok(mut map) = hashes.lock() {
                    map.entry(hash)
                        .or_insert_with(Vec::new)
                        .push(path.clone());
                } else {
                    error!("Failed to acquire lock when processing: {}", path.display());
                }
            }
            Err(err) => {
                error!("Failed to compute hash for {}: {}", path.display(), err);
            }
        }
    });


    match &args.output {
        Some(output) => save_duplicates_to_file(&args.dir, &output, &hashes),
        None => print_duplicates(&args.dir, &hashes)
    };

    Ok(())
}

fn print_duplicates(path: &PathBuf, dups: &Arc<Mutex<BTreeMap<Vec<u8>, Vec<PathBuf>>>>) {
    info!(
        "duplicate files found in {}:\n",
        path.as_os_str().to_str().unwrap()
    );

    if let Ok(mut hashes) = dups.lock() {
        for files in hashes.values_mut() {
            files.sort();
        }

        for (hash, paths) in hashes.iter_mut() {
            if paths.len() > 1 {
                println!("duplicate files (sha3-256: {})", hex::encode(hash));
                paths.sort();
                for path in paths {
                    println!("    - {}", path.display());
                }
                print!("\n");
            }
        }
    } else {
        error!("failed to acquire lock when processing: {}", path.display());
    }
}

fn save_duplicates_to_file(path: &PathBuf, output: &str, dups: &Arc<Mutex<BTreeMap<Vec<u8>, Vec<PathBuf>>>>) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(output)
        .unwrap();

    file.write_all(
        format!(
            "duplicate files found in {}:\n\n",
            path.as_os_str().to_str().unwrap()
        )
        .as_bytes(),
    )
    .expect("write to file failed");

    if let Ok(mut hashes) = dups.lock() {
        for files in hashes.values_mut() {
            files.sort();
        }

        for (hash, paths) in hashes.iter_mut() {
            if paths.len() > 1 {
                file.write_all(format!("duplicate files (sha3-256: {}\n)", hex::encode(hash)).as_bytes())
                    .expect("write to file failed");
                paths.sort();
                for path in paths {
                    file.write_all(format!("    - {}\n", path.display()).as_bytes())
                        .expect("write to file failed");
                }
                file.write_all(format!("{}", "\n").as_bytes())
                    .expect("write to file failed");
            }
        }
    } else {
        error!("failed to acquire lock when processing: {}", path.display());
    }

    info!(
        "duplicate files check finished, please confirm at: {}\n",
        output
    );
}
