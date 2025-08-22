use crate::command::CommandExec;
use crate::enumerate::file::FileHashType;
use crate::error::Error;
use crate::util::file::find_dup_files;
use crate::util::hash::compute_file_hash;
use clap::Args;
use log::{error, info, warn};
use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;
use std::path::PathBuf;

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

    #[arg(
        short,
        long,
        help = "hash algorithm to compute the file digest: md5|sha1|sha256|sha3-224|sha3-256|sha3-384|sha3-512, default is sha3-256",
        required = false,
        default_value = "sha3-256"
    )]
    pub digest: FileHashType,

    #[arg(
        long,
        help = "keep the specified files, default is all files",
        required = false,
        value_delimiter = ','
    )]
    pub protect: Option<Vec<PathBuf>>,
}

impl CommandExec for DupDelArgs {
    fn exec(&self) -> Result<(), Error> {
        // 找到重复文件使用了hash算法，在删除时，用另一种hash算法校验是否仍一致
        let second_digest = if self.digest == FileHashType::MD5 {
            FileHashType::SHA1
        } else {
            FileHashType::MD5
        };

        let protected_list = match &self.protect {
            Some(list) => list.clone(),
            None => vec![],
        };

        let result = find_dup_files(&self.dir, self.recursive, &self.digest)?;

        result.par_iter().for_each(|(hash, files)| {
            info!(
                "delete duplicates files ({}: {})",
                self.digest,
                hex::encode(hash)
            );

            let mut protected = Vec::new();
            let mut not_protected = Vec::new();

            for file in files {
                if is_protectd(&protected_list, file) {
                    protected.push(file.clone());
                } else {
                    not_protected.push(file.clone());
                }
            }

            // 所有文件皆受保护，不删除文件
            if not_protected.is_empty() {
                info!("all files is in protected, no files to delete");
                return;
            }

            // 所有文件都不受保护，保留一个文件，其余删除
            if protected.is_empty() {
                info!("all files is not in protected, keep one file and delete others");
                for i in 1..not_protected.len() {
                    match delete_file(
                        &not_protected[0],
                        &not_protected[i],
                        &self.digest,
                        &second_digest,
                    ) {
                        Ok(_) => {
                            info!("file {} deleted", not_protected[i].display());
                        }
                        Err(e) => {
                            error!("delete file {} failed: {}", not_protected[i].display(), e);
                        }
                    }
                }
            } else {
                // 既存在受保护文件，又存在不受保护文件，不受保护文件都删除
                info!(
                    "contains {} protected files, delete {} files that not in protected",
                    protected.len(),
                    not_protected.len()
                );
                for file in not_protected {
                    match delete_file(&protected[0], &file, &self.digest, &second_digest) {
                        Ok(_) => {
                            info!("file {} deleted", file.display());
                        }
                        Err(e) => {
                            error!("delete file {} failed: {}", file.display(), e);
                        }
                    }
                }
            }
        });

        info!("duplicates files deleted");

        Ok(())
    }
}

fn is_protectd(protected_list: &Vec<PathBuf>, file: &PathBuf) -> bool {
    for protected in protected_list {
        if file.starts_with(protected) {
            return true;
        }
    }
    false
}

// delete file if sha3-256 and md5 is same
fn delete_file(
    exist: &PathBuf,
    to_delete: &PathBuf,
    digest: &FileHashType,
    digest2: &FileHashType,
) -> Result<(), Error> {
    let hash_exist = compute_file_hash(to_delete, &FileHashType::MD5)?;
    let hash_delete = compute_file_hash(exist, &FileHashType::MD5)?;

    if hash_exist == hash_delete {
        match std::fs::remove_file(to_delete) {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::CustomError(format!(
                "delete file {:?} failed: {}",
                to_delete, err
            ))),
        }
    } else {
        warn!(
            "files[path: {:?}, {}: {}] and file[path: {:?}, {}: {}] are the same in {}, but different in {}",
            exist,
            digest2,
            hex::encode(hash_exist),
            to_delete,
            digest2,
            hex::encode(hash_delete),
            digest,
            digest2
        );
        Err(Error::CustomError(format!(
            "file {:?} and {:?} are the same in {}, but different in {}",
            exist, to_delete, digest, digest2
        )))
    }
}
