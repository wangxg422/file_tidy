use std::fs::File;
use std::io::Read;
use sha3::{Sha3_256};
use md5::{Digest, Md5};
use sha1::Sha1;
use crate::enumerate::file::NamingRule;
use crate::error::Error;
use crate::error::Error::CustomError;

pub fn compute_file_hash(hash_type: &NamingRule, path: &std::path::Path) -> Result<Vec<u8>, Error> {
    let mut file = File::open(path)?;

    let hash_result = match hash_type {
        NamingRule::MD5 => compute_hash(&mut file, Md5::new()),
        NamingRule::SHA1 => compute_hash(&mut file, Sha1::new()),
        NamingRule::SHA256 => compute_hash(&mut file, Sha3_256::new()),
        _ => Err(CustomError("".to_string()))
    }?;

    Ok(hash_result)
}

fn compute_hash<H: Digest>(file: &mut File, mut hasher: H) -> Result<Vec<u8>, Error> {
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