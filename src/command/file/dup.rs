use std::collections::BTreeMap;
use std::path::PathBuf;
use clap::Args;
use crate::command::{CommandArgs, CommandExec};

pub struct DupCommand {}

impl CommandExec for DupCommand {
    fn exec(&self, _args: &DuplicatesArgs) {
        let mut hashes: BTreeMap<Vec<u8>, Vec<PathBuf>> = BTreeMap::new();

        for entry in WalkDir::new(root_dir.clone()) {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                // 计算文件哈希
                let hash = compute_file_hash(path)?;

                // 将文件路径添加到对应哈希的列表中
                hashes.entry(hash)
                    .or_insert_with(Vec::new)
                    .push(path.to_path_buf());
            }
        }

        print_duplicates(&root_dir, &mut hashes)
    }
}


fn compute_file_hash(path: &std::path::Path) -> Result<Vec<u8>> {
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

fn print_duplicates(path: &str, hashes: &mut BTreeMap<Vec<u8>, Vec<PathBuf>>) -> Result<()> {
    let detail = format!("{}/{}", remove_trailing_slash(&path), "__duplicate_file.txt");

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&detail)?;

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

    println!("Duplicate files check finished, please confirm at: {}\n", &detail);
    Ok(())
}

fn remove_trailing_slash(path: &str) -> &str {
    if path.ends_with('/') {
        &path[..path.len() - 1]
    } else {
        path
    }
}


#[derive(Args)]
pub struct DuplicatesArgs {
    #[arg(short, long, help = "path of files")]
    pub dir: String,

    #[arg(short, long, help = "where to save the duplicate files, default is ")]
    pub output: String,
}

impl CommandArgs for DuplicatesArgs {}