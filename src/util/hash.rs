use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use md5::{Digest, Md5};
use md5::digest::DynDigest;
use sha1::Sha1;
use sha2::Sha256;
use sha3::{Sha3_224, Sha3_256, Sha3_384, Sha3_512};
use crate::enumerate::file::FileHashType;
use crate::error::Error;

pub fn compute_file_hash(path: &Path, hash_type: &FileHashType) -> Result<Vec<u8>, Error> {
    let mut hasher: Box<dyn DynDigest> = match hash_type {
        FileHashType::MD5 => Box::new(Md5::new()),
        FileHashType::SHA1 => Box::new(Sha1::new()),
        FileHashType::SHA256 => Box::new(Sha256::new()),
        FileHashType::SHA3_224 => Box::new(Sha3_224::new()),
        FileHashType::SHA3_256 => Box::new(Sha3_256::new()),
        FileHashType::SHA3_384 => Box::new(Sha3_384::new()),
        FileHashType::SHA3_512 => Box::new(Sha3_512::new()),
    };

    let mut reader = BufReader::new(File::open(path)?);
    let mut buffer = [0; 8192];

    while let Ok(n) = reader.read(&mut buffer) {
        if n == 0 { break; }
        hasher.update(&buffer[..n]);
    }

    Ok(hasher.finalize().to_vec())
}