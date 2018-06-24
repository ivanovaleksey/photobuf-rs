#[macro_use]
extern crate quicli;

use quicli::prelude::*;

use std::collections::HashMap;
use std::path::PathBuf;
use std::{fs, io};

/// Sync photos with Yandex.Disk
#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(subcommand)]
    cmd: Command,
    #[structopt(flatten)]
    verbosity: Verbosity,
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "pack", about = "Pack items from `source` into by-year folders in `destination`")]
    Pack {
        #[structopt(parse(from_os_str))]
        source: PathBuf,
        #[structopt(parse(from_os_str))]
        destination: PathBuf,
    },
}

main!(|args: Cli, log_level: verbosity| {
    match args.cmd {
        Command::Pack {
            source,
            destination,
        } => pack(source, destination),
    };
});

fn pack(src: PathBuf, dst: PathBuf) -> io::Result<()> {
    let mut hash: HashMap<u16, Vec<String>> = HashMap::new();

    let entries = fs::read_dir(&src).unwrap();
    for entry in entries {
        let entry = entry?;
        let name = entry.file_name();
        let name = name.into_string().unwrap();
        let year: u16 = name[..4].parse().unwrap();

        hash.entry(year)
            .and_modify(|list| list.push(name.clone()))
            .or_insert_with(|| vec![name]);
    }

    if dst.exists() {
        fs::remove_dir_all(&dst).unwrap();
    }

    for (year, names) in &hash {
        let year = year.to_string();
        fs::create_dir_all(dst.join(&year)).unwrap();

        for name in names.iter() {
            let from = src.join(name);
            let to = dst.join(&year).join(name);

            debug!("Copy from: {:?}; to: {:?}", from, to);
            fs::copy(from, to).unwrap();
        }
    }

    Ok(())
}
