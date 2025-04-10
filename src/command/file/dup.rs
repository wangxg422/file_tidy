use clap::Args;

#[derive(Args)]
pub struct DuplicatesArgs {
    #[arg(short, long, help = "path of files")]
    pub dir: String,

    #[arg(short, long, help = "where to save the duplicate files, default is ")]
    pub output: String,
}