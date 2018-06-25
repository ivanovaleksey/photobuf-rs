use quicli::prelude::*;

use std::path::PathBuf;

pub mod pack;
pub mod ping;
pub mod sync;

/// Sync photos with Yandex.Disk
#[derive(Debug, StructOpt)]
pub struct Cli {
    #[structopt(subcommand)]
    pub cmd: Command,
    #[structopt(flatten)]
    pub verbosity: Verbosity,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(
        name = "pack", about = "Pack items from `source` into by-year folders in `destination`"
    )]
    Pack {
        #[structopt(parse(from_os_str))]
        source: PathBuf,
        #[structopt(parse(from_os_str))]
        destination: PathBuf,
    },
    #[structopt(name = "ping", about = "Ping Yandex.Disk API to make sure everything is fine")]
    Ping,
    #[structopt(name = "sync", about = "Sync items from `directory` with Yandex.Disk")]
    Sync {
        #[structopt(parse(from_os_str))]
        directory: PathBuf,
    },
}
