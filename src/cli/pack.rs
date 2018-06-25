use quicli::prelude::*;

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub fn call(src: PathBuf, dst: PathBuf) -> Result<()> {
    let mut hash: HashMap<u16, Vec<String>> = HashMap::new();

    let entries = fs::read_dir(&src)?;
    for entry in entries {
        let entry = entry?;
        let name = entry.file_name();
        let name = name
            .into_string()
            .map_err(|_| err_msg("Cannot convert OsString into String"))?;
        let year: u16 = name[..4].parse()?;

        hash.entry(year)
            .and_modify(|list| list.push(name.clone()))
            .or_insert_with(|| vec![name]);
    }

    if dst.exists() {
        fs::remove_dir_all(&dst)?;
    }

    for (year, names) in &hash {
        let year = year.to_string();
        fs::create_dir_all(dst.join(&year))?;

        for name in names.iter() {
            let from = src.join(name);
            let to = dst.join(&year).join(name);

            debug!("Copy from: {:?}; to: {:?}", from, to);
            fs::copy(from, to)?;
        }
    }

    Ok(())
}
