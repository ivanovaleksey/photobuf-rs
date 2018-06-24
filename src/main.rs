use std::io::Result;

fn main() {
    if let Err(e) = try_main() {
        println!("Error: {}", e);
    } else {
        println!("Done");
    }
}

fn try_main() -> Result<()> {
    use std::fs;
    use std::collections::HashMap;

    let mut hash: HashMap<u16, Vec<String>> = HashMap::new();

    let entries = fs::read_dir("./buf/in").unwrap();
    for entry in entries {
        let entry = entry?;
        let name = entry.file_name();
        let name = name.into_string().unwrap();
        let year: u16 = name[..4].parse().unwrap();

        hash.entry(year)
            .and_modify(|list| list.push(name.clone()))
            .or_insert_with(|| vec![name]);
    }

    fs::remove_dir_all("./buf/out").unwrap();
    for (year, names) in &hash {
        fs::create_dir_all(format!("./buf/out/{}", year)).unwrap();

        for name in names.iter() {
            let from = format!("./buf/in/{}", name);
            let to = format!("./buf/out/{}/{}", year, name);
            fs::copy(from, to).unwrap();
        }
    }

    Ok(())
}
