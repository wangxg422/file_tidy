use crate::command::CommandExec;
use crate::enumerate::file::FileHashType;
use crate::error::Error;
use crate::util::hash::compute_file_hash;
use clap::Args;
use log::{info, warn};
use rayon::prelude::*;
use std::collections::BTreeMap;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Args)]
pub struct DupListArgs {
    #[arg(long, help = "path of files", required = true)]
    pub dir: PathBuf,

    #[arg(
        short,
        long,
        help = "where to save the duplicate files",
        required = false
    )]
    pub output: Option<String>,

    #[arg(
        short,
        long,
        help = "whether to recursively search for duplicate files",
        required = false
    )]
    pub recursive: bool,

    #[arg(
        short,
        long,
        help = "hash algorithm to compute the file digest: md5|sha1|sha256|sha3-224|sha3-256|sha3-384|sha3-512, default is sha3-256",
        required = false,
        default_value = "sha3-256"
    )]
    pub digest: FileHashType,
}

impl CommandExec for DupListArgs {
    fn exec(&self) -> Result<(), Error> {
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

        // 按照文件大小分组，文件大小不同，一定不是同一文件,只有文件大小相同的文件才可能是同一文件
        let mut size_groups: BTreeMap<u64, Vec<PathBuf>> = BTreeMap::new();
        for path in entries {
            if let Ok(meta) = fs::metadata(&path) {
                size_groups.entry(meta.len()).or_default().push(path);
            } else {
                warn!("Failed to get metadata for {}", path.display());
            }
        }

        let result: BTreeMap<Vec<u8>, Vec<PathBuf>> = size_groups
            .into_par_iter()
            .filter(|(_, files)| files.len() > 1)
            .map(|(_, files)| {
                let mut map: BTreeMap<Vec<u8>, Vec<PathBuf>> = BTreeMap::new();
                for file in files {
                    if let Ok(h) = compute_file_hash(&file, &self.digest) {
                        map.entry(h).or_insert_with(|| Vec::new()).push(file);
                    } else {
                        warn!("Failed to compute hash for file {}", file.display());
                    }
                }

                map
            })
            .reduce(
                || BTreeMap::new(),
                |mut acc, local| {
                    for (h, paths) in local {
                        acc.entry(h).or_default().extend(paths);
                    }
                    acc
                },
            )
            .into_iter()
            .filter(|(_, files)| files.len() > 1)
            .map(|(h, mut files)| {
                files.sort();
                (h, files)
            })
            .collect();

        if let Some(output) = &self.output {
            save_duplicates_to_file(&self.dir, output, &result);
        } else {
            print_duplicates(&self.dir, result)
        }
        Ok(())
    }
}

fn print_duplicates(path: &PathBuf, hashes: BTreeMap<Vec<u8>, Vec<PathBuf>>) {
    info!(
        "duplicate files found in {}:\n",
        path.as_os_str().to_str().unwrap()
    );

    for (hash, paths) in hashes.iter() {
        if paths.len() > 1 {
            println!("duplicate files (sha3-256: {})", hex::encode(hash));
            for path in paths {
                println!("    - {}", path.display());
            }
            print!("\n");
        }
    }
}

fn save_duplicates_to_file(path: &PathBuf, output: &str, hashes: &BTreeMap<Vec<u8>, Vec<PathBuf>>) {
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

    for (hash, paths) in hashes.iter() {
        if paths.len() > 1 {
            file.write_all(
                format!("duplicate files (sha3-256: {}\n)", hex::encode(hash)).as_bytes(),
            )
            .expect("write to file failed");

            for path in paths {
                file.write_all(format!("    - {}\n", path.display()).as_bytes())
                    .expect("write to file failed");
            }
            file.write_all(format!("{}", "\n").as_bytes())
                .expect("write to file failed");
        }
    }

    info!(
        "duplicate files check finished, please confirm at: {}\n",
        output
    );
}
