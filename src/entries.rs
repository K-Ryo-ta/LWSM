use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;

pub fn read_entries(path: &Path) -> Result<Vec<DirEntry>, io::Error> {
    let mut entries = fs::read_dir(path)?
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort_by_key(|entry| entry.file_name());

    Ok(entries)
}