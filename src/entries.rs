use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;

pub fn read_entries(path: &Path) -> Result<Vec<DirEntry>, io::Error> {
    let mut entries = fs::read_dir(path)?
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort_by_key(|entry| entry.file_name());

    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    fn temp_dir(prefix: &str) -> PathBuf {
        std::env::temp_dir().join(format!("{prefix}_{}", std::process::id()))
    }

    #[test]
    fn read_entries_returns_sorted_file_names() {
        let dir = temp_dir("lwsm_entries");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        fs::File::create(dir.join("b.txt")).unwrap();
        fs::File::create(dir.join("a.txt")).unwrap();
        fs::create_dir_all(dir.join("z_dir")).unwrap();

        let entries = read_entries(&dir).unwrap();
        let names: Vec<String> = entries
            .iter()
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .collect();

        assert_eq!(names, vec!["a.txt", "b.txt", "z_dir"]);
        let _ = fs::remove_dir_all(&dir);
    }
}