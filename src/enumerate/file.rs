use clap::ValueEnum;

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum FileHashType {
    MD5, // 128bit
    SHA1, // 160bit
    SHA256, // 256bit
    SHA3_224,
    SHA3_256,
    SHA3_384,
    SHA3_512,
    //BLAKE2b,
    //BLAKE2s
}