use std::fs::DirEntry;
use std::io;

pub fn print_entries(entries: Vec<DirEntry>) -> Result<(), io::Error> {
    for entry in entries {
        print_entry(&entry)?;
    }

    Ok(())
}

fn print_entry(entry: &DirEntry) -> Result<(), io::Error> {
    let file_name = entry.file_name();
    let name = file_name.to_string_lossy();

    let file_type = entry.file_type()?;

    if file_type.is_dir() {
        println!("{}/", name);
    } else {
        println!("{}", name);
    }

    Ok(())
}