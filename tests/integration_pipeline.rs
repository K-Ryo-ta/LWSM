use lwsm::config::{Config, Mode};
use lwsm::entries;
use lwsm::search;
use std::fs;
use std::path::PathBuf;

fn fixture(names: &[&str]) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("lwsm_m_{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    for name in names {
        let path = dir.join(name);
        if name.ends_with('/') {
            fs::create_dir_all(path).unwrap();
        } else {
            fs::File::create(path).unwrap();
        }
    }
    dir
}

fn matched_names(dir: &PathBuf, config: &Config) -> Vec<String> {
    let entries = entries::read_entries(dir).unwrap();
    search::filter_entries(entries, config)
        .into_iter()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect()
}

#[test]
fn pipeline_word_match_filters_entries() {
    let dir = fixture(&["alpha.txt", "beta.txt", "gamma-rust.txt"]);
    let config = Config {
        mode: Mode::WordMatch,
        query: "rust".into(),
        path: dir.clone(),
    };

    assert_eq!(matched_names(&dir, &config), vec!["gamma-rust.txt"]);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn pipeline_sentence_search_filters_entries() {
    let dir = fixture(&["hello_world.txt", "notes.txt"]);
    let config = Config {
        mode: Mode::SentenceSearch,
        query: "hello world".into(),
        path: dir.clone(),
    };

    assert_eq!(matched_names(&dir, &config), vec!["hello_world.txt"]);
    let _ = fs::remove_dir_all(&dir);
}
