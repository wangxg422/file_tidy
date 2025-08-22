use crate::command::CommandExec;
use crate::enumerate::file::FileHashType::{
    MD5, SHA1, SHA3_224, SHA3_256, SHA3_384, SHA3_512, SHA256,
};
use crate::enumerate::sort::FileSort;
use crate::error::Error;
use crate::util::hash::compute_file_hash;
use clap::Args;
use log::{debug, error, info};
use rayon::prelude::*;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Args)]
pub struct RenameArgs {
    #[arg(long, help = "path of files", required = true)]
    pub dir: PathBuf,

    /// Naming rule to apply (choose one)
    #[command(flatten)]
    pub naming_rule: NamingRuleArgs,

    #[arg(long = "upper", help = "uppercase file name", required = false)]
    pub upper: bool,

    #[arg(long = "lower", help = "lowercase file name", required = false)]
    pub lower: bool,

    #[arg(
        long = "upper-ext",
        help = "uppercase file extension",
        required = false
    )]
    pub upper_ext: bool,

    #[arg(
        long = "lower-ext",
        help = "lowercase file extension",
        required = false
    )]
    pub low_ext: bool,

    #[arg(
        long = "seq-len",
        help = "length of sequence, default 6",
        required = false,
        default_value = "6"
    )]
    pub seq_len: usize,

    #[command(flatten)]
    pub seq_sort: SeqSortArgs,

    #[arg(long = "prefix", help = "prefix of file name", required = false)]
    pub prefix: Option<String>,

    #[arg(long = "suffix", help = "suffix of file name", required = false)]
    pub suffix: Option<String>,

    #[arg(
        long = "ignore",
        help = "ignore file and dir, hidden file (which start with '.' is ignored default)",
        required = false
    )]
    pub ignore: Vec<String>,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
pub struct NamingRuleArgs {
    #[arg(long = "seq", help = "rename file by sequence", required = false)]
    pub sequence: bool,

    #[arg(long = "md5", help = "rename file by md5 value", required = false)]
    pub md5: bool,

    #[arg(long = "sha1", help = "rename file by sha1 value", required = false)]
    pub sha1: bool,

    #[arg(
        long = "sha256",
        help = "rename file by sha256 value",
        required = false
    )]
    pub sha256: bool,

    #[arg(
        long = "sha3-224",
        help = "rename file by sha3-224 value",
        required = false
    )]
    pub sha3_224: bool,

    #[arg(
        long = "sha3-256",
        help = "rename file by sha3-256 value",
        required = false
    )]
    pub sha3_256: bool,

    #[arg(
        long = "sha3-384",
        help = "rename file by sha3-384 value",
        required = false
    )]
    pub sha3_384: bool,

    #[arg(
        long = "sha3-512",
        help = "rename file by sha3-512 value",
        required = false
    )]
    pub sha3_512: bool,
}

#[derive(Args, Debug)]
pub struct SeqSortArgs {
    #[arg(
        long = "sort",
        help = "sort of file when rename file by sequence, one of name|size|time, default is name",
        required = false,
        default_value = "name"
    )]
    pub sort: FileSort,

    #[arg(
        long = "asc",
        help = "if `--sort` is setted, set asc to ",
        required = false,
        requires = "sort",
        group = "rename-seq-sort"
    )]
    pub asc: bool,

    #[arg(
        long = "desc",
        help = "if `--sort` is setted, set desc to ",
        required = false,
        requires = "sort",
        group = "rename-seq-sort"
    )]
    pub desc: bool,
}

impl CommandExec for RenameArgs {
    fn exec(&self) -> Result<(), Error> {
        if !self.dir.exists() {
            error!("path does not exist: {}", self.dir.display());
            return Err(Error::CustomError(format!(
                "path does not exist: {}",
                self.dir.display()
            )));
        }

        let naming_by = if self.naming_rule.md5 {
            info!("rename file by md5");
            MD5
        } else if self.naming_rule.sha1 {
            info!("rename file by sha1");
            SHA1
        } else if self.naming_rule.sha256 {
            info!("rename file by sha256");
            SHA256
        } else if self.naming_rule.sha3_224 {
            info!("rename file by sha3-224");
            SHA3_224
        } else if self.naming_rule.sha3_256 {
            info!("rename file by sha3-256");
            SHA3_256
        } else if self.naming_rule.sha3_384 {
            info!("rename file by sha3-384");
            SHA3_384
        } else if self.naming_rule.sha3_512 {
            info!("rename file by sha3-512");
            SHA3_512
        } else if self.naming_rule.sequence {
            info!("rename file by sequence");
            return self.rename_by_seq();
        } else {
            error!("unknown naming rule");
            return Err(Error::CustomError("unknown naming rule".to_string()));
        };

        let entries: Vec<_> = WalkDir::new(&self.dir)
            .max_depth(1)
            .into_iter()
            .filter_entry(|e| {
                // 目录或文件名不是隐藏的才进入
                !e.file_name().to_string_lossy().starts_with('.')
            })
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file()) // 只要文件
            .map(|e| e.path().to_path_buf())
            .collect();

        entries.par_iter().for_each(|file| {
            if let Ok(hash) = compute_file_hash(file, &naming_by) {
                let new_name = if self.upper {
                    hex::encode_upper(hash)
                } else {
                    hex::encode(hash)
                };

                let new_path = match file.extension() {
                    Some(ext) => {
                        let mut ext_name = ext.to_str().unwrap().to_string();
                        if self.upper_ext {
                            ext_name = ext_name.to_ascii_uppercase();
                        }
                        if self.low_ext {
                            ext_name = ext_name.to_ascii_lowercase();
                        }

                        let prefix = &self.prefix.as_deref().unwrap_or("");
                        let suffix = &self.suffix.as_deref().unwrap_or("");

                        file.with_file_name(format!(
                            "{}{}{}.{}",
                            prefix, &new_name, suffix, ext_name
                        ))
                    }
                    None => file.with_file_name(&new_name),
                };

                info!("renamed {:?} to {:?}", file, new_path);
                fs::rename(&file, &new_path).unwrap_or_else(|err| {
                    error!(
                        "Failed to rename file {:?} to {:?}: {}",
                        file, new_path, err
                    );
                });
            } else {
                error!("Failed to compute hash for file {:?}", file);
            }
        });

        info!("file renamed finished");
        Ok(())
    }
}

impl RenameArgs {
    fn rename_by_seq(&self) -> Result<(), Error> {
        let mut entries: Vec<_> = WalkDir::new(&self.dir)
            .max_depth(1)
            .into_iter()
            .filter_entry(|e| {
                // 目录或文件名不是隐藏的才进入
                !e.file_name().to_string_lossy().starts_with('.')
            })
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file()) // 只要文件
            .map(|e| e.path().to_path_buf())
            .collect();

        match self.seq_sort.sort {
            FileSort::Name => {
                if self.seq_sort.desc {
                    debug!("sort by file name, desc");
                    entries.sort_by(|a, b| b.cmp(a))
                } else {
                    debug!("sort by file name, asc");
                    entries.sort_by(|a, b| a.cmp(b))
                }
            }
            FileSort::Size => {
                if self.seq_sort.desc {
                    debug!("sort by file size, desc");
                    entries.sort_by(|a, b| {
                        let a = a.metadata().unwrap().len();
                        let b = b.metadata().unwrap().len();
                        b.cmp(&a)
                    })
                } else {
                    debug!("sort by file size, asc");
                    entries.sort_by(|a, b| {
                        let a = a.metadata().unwrap().len();
                        let b = b.metadata().unwrap().len();
                        a.cmp(&b)
                    })
                }
            }
            FileSort::Time => {
                if self.seq_sort.desc {
                    debug!("sort by last update time, desc");
                    entries.sort_by(|a, b| {
                        let a = a.metadata().unwrap().modified().unwrap();
                        let b = b.metadata().unwrap().modified().unwrap();
                        b.cmp(&a)
                    })
                } else {
                    debug!("sort by last update time, desc");
                    entries.sort_by(|a, b| {
                        let a = a.metadata().unwrap().modified().unwrap();
                        let b = b.metadata().unwrap().modified().unwrap();
                        a.cmp(&b)
                    })
                }
            }
        }

        // 使用 enumerate 分配序号，避免共享可变 state
        entries.par_iter().enumerate().for_each(|(i, file)| {
            let new_name = format!("{:0width$}", i + 1, width = self.seq_len);

            let new_path = match file.extension().and_then(|s| s.to_str()) {
                Some(ext) if !ext.is_empty() => {
                    let ext_name = if self.upper_ext {
                        ext.to_ascii_uppercase()
                    } else if self.low_ext {
                        ext.to_ascii_lowercase()
                    } else {
                        ext.to_string()
                    };

                    let prefix = &self.prefix.as_deref().unwrap_or("");
                    let suffix = &self.suffix.as_deref().unwrap_or("");

                    file.with_file_name(format!("{}{}{}.{}", prefix, new_name, suffix, ext_name))
                }
                _ => file.with_file_name(&new_name),
            };

            info!("renamed {:?} to {:?}", file, new_path);
            if let Err(err) = fs::rename(file, &new_path) {
                error!(
                    "Failed to rename file {:?} to {:?}: {}",
                    file, new_path, err
                );
            }
        });

        Ok(())
    }
}
