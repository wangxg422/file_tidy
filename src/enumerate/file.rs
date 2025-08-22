use std::fmt;

use clap::ValueEnum;

#[derive(Copy, Clone, Debug, ValueEnum, PartialEq)]
pub enum FileHashType {
    MD5,    // 128bit
    SHA1,   // 160bit
    SHA256, // 256bit
    SHA3_224,
    SHA3_256,
    SHA3_384,
    SHA3_512,
    //BLAKE2b,
    //BLAKE2s
}

impl fmt::Display for FileHashType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileHashType::MD5 => write!(f, "md5"),
            FileHashType::SHA1 => write!(f, "sha1"),
            FileHashType::SHA256 => write!(f, "sha256"),
            FileHashType::SHA3_224 => write!(f, "sha3-224"),
            FileHashType::SHA3_256 => write!(f, "sha3-256"),
            FileHashType::SHA3_384 => write!(f, "sha3-384"),
            FileHashType::SHA3_512 => write!(f, "sha3-512"),
        }
    }
}
