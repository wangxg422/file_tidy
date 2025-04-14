use std::collections::BTreeMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use sha3::{Digest, Sha3_256};
use walkdir::WalkDir;
use crate::command::file::DupArgs;
use crate::error::Error;

pub fn exec(args: &DupArgs) {
    let mut hashes: BTreeMap<Vec<u8>, Vec<PathBuf>> = BTreeMap::new();

    for entry in WalkDir::new(&args.dir) {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            let hash = compute_file_hash(path).unwrap();

            hashes.entry(hash)
                .or_insert_with(Vec::new)
                .push(path.to_path_buf());
        }
    }

    if args.output {
        save_duplicates_to_file(&args.dir, &mut hashes);
    } else {
        print_duplicates(&args.dir, &mut hashes);
    }
}

fn compute_file_hash(path: &std::path::Path) -> Result<Vec<u8>, Error> {
    let mut file = File::open(path)?;
    let mut hasher = Sha3_256::new();
    let mut buffer = [0; 1024];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(hasher.finalize().to_vec())
}

fn print_duplicates(path: &PathBuf, hashes: &mut BTreeMap<Vec<u8>, Vec<PathBuf>>) {
    println!("Duplicate files in {}:\n\n", path);

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

    let file= "__duplicates.txt";

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file).unwrap();

    file.write_all(format!("Duplicate files in {}:\n\n", path).as_bytes()).expect("write to file failed");

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

    println!("Duplicate files check finished, please confirm at: {}\n", &file);
}