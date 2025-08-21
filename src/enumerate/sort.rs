use clap::ValueEnum;

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum FileSort {
    Name,
    Time,
    Size
}