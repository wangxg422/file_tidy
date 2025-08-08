mod command;
mod enumerate;
mod error;
mod handle;
mod util;

use crate::command::Commands;
use clap::Parser;
use log::{debug, error, info, LevelFilter};
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::Config;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;

#[derive(Parser)]
#[command(name = "file-tidy", version = "v0.1.0", about = "A cli sample")]
#[command(help_template = "{bin} {version}

{about}

USAGE:

{usage}

 {all-args}")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long)]
    debug: bool,

    #[arg(short, long, global = true)]
    verbose: bool,
}

fn main() {
    // 创建 stdout appender
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S%.3f)} {M} {l} - {m}{n}",
        )))
        .target(Target::Stdout)
        .build();

    // 配置
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Debug)).unwrap();

    // 初始化 log4rs
    log4rs::init_config(config).unwrap();

    let cli = Cli::parse();

    if cli.debug {
        debug!("Debug mode enabled")
    }

    match cli.command.exec() {
        Ok(()) => {
            info!("executed finished");
        }
        Err(err) => error!("Error: {}", err),
    }
}
