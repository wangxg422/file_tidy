use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

use log::warn;
use rayon::prelude::*;
use walkdir::WalkDir;

use crate::{enumerate::file::FileHashType, error::Error, util::hash::compute_file_hash};

// 用指定hash算法计算文件哈希值，找出指定目录中存在的重复文件。重复文件的判定标准：
// 1.文件大一样
// 2.文件哈希值一样
pub fn find_dup_files(
    dir: &Path,
    recursive: bool,
    digest: &FileHashType,
) -> Result<BTreeMap<Vec<u8>, Vec<PathBuf>>, Error> {
    let mut walkdir = WalkDir::new(dir);

    if !recursive {
        walkdir = walkdir.max_depth(1);
    }

    let entries: Vec<_> = walkdir
        .into_iter()
        .filter_entry(|e| {
            // 忽略隐藏目录及隐藏目录中的文件，忽略隐藏文件
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
                if let Ok(h) = compute_file_hash(&file, &digest) {
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

    Ok(result)
}
