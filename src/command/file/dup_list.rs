use crate::command::CommandExec;
use crate::enumerate::file::FileHashType;
use crate::error::Error;
use crate::util::file::find_dup_files;
use clap::Args;
use log::{info};
use std::collections::BTreeMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

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
        let result = find_dup_files(&self.dir, self.recursive, &self.digest)?;

        if let Some(output) = &self.output {
            save_duplicates_to_file(&self.dir, output, &result, &self.digest);
        } else {
            print_duplicates(&self.dir, result, &self.digest)
        }
        Ok(())
    }
}

fn print_duplicates(
    path: &PathBuf,
    hashes: BTreeMap<Vec<u8>, Vec<PathBuf>>,
    digest: &FileHashType,
) {
    info!(
        "duplicate files found in {}:\n",
        path.as_os_str().to_str().unwrap()
    );

    for (hash, paths) in hashes.iter() {
        if paths.len() > 1 {
            println!("duplicate files ({}: {})", digest, hex::encode(hash));
            for path in paths {
                println!("    - {}", path.display());
            }
            print!("\n");
        }
    }
}

fn save_duplicates_to_file(
    path: &PathBuf,
    output: &str,
    hashes: &BTreeMap<Vec<u8>, Vec<PathBuf>>,
    digest: &FileHashType,
) {
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
                format!("duplicate files ({}: {})\n", digest, hex::encode(hash)).as_bytes(),
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
