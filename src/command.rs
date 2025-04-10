pub mod file;

use clap::Subcommand;
use crate::command::file::FileCommand;

#[derive(Subcommand)]
pub enum Commands {
    #[command(subcommand, about="manage files")]
    File(FileCommand),
    Delete { id: u32 },
}

impl Commands {
    pub fn exec(&self) {
        match self {
            Commands::File(file_cmd) => file_cmd.exec(),
            Commands::Delete { id } => println!("Deleting {}", id),
        }
    }
}