use crate::command::file::DupArgs;
use crate::enumerate::file::FileHashType;
use crate::util;
use std::collections::BTreeMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn handle(args: &DupArgs) {
    let mut hashes: BTreeMap<Vec<u8>, Vec<PathBuf>> = BTreeMap::new();

    for entry in WalkDir::new(&args.dir) {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            let hash = util::compute_file_hash(&FileHashType::MD5, path).unwrap();

            hashes
                .entry(hash)
                .or_insert_with(Vec::new)
                .push(path.to_path_buf());
        }
    }

    if args.output.is_empty() {
        save_duplicates_to_file(&args.dir, &mut hashes);
    } else {
        print_duplicates(&args.dir, &mut hashes);
    }
}

fn print_duplicates(path: &PathBuf, hashes: &mut BTreeMap<Vec<u8>, Vec<PathBuf>>) {
    println!(
        "Duplicate files in {}:\n\n",
        path.as_os_str().to_str().unwrap()
    );

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
    let detail = "__duplicates.txt";

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(detail)
        .unwrap();

    file.write_all(
        format!(
            "Duplicate files in {}:\n\n",
            path.as_os_str().to_str().unwrap()
        )
        .as_bytes(),
    )
    .expect("write to file failed");

    for files in hashes.values_mut() {
        files.sort();
    }

    for (hash, paths) in hashes {
        if paths.len() > 1 {
            file.write_all(format!("Duplicate files sha3: {}\n", hex::encode(hash)).as_bytes())
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

    println!(
        "Duplicate files check finished, please confirm at: {}\n",
        detail
    );
}
