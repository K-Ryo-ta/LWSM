use lwsm::config::{Config, Mode};
use lwsm::entries;
use lwsm::search;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

fn fixture(label: &str, names: &[&str]) -> PathBuf {
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("lwsm_pipe_{label}_{}_{n}", std::process::id()));
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

fn matched_names(dir: &Path, config: &Config) -> Vec<String> {
    let entries = entries::read_entries(dir).unwrap();
    search::filter_entries(entries, config)
        .into_iter()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect()
}

#[test]
fn pipeline_word_match_filters_entries() {
    let dir = fixture("word", &["alpha.txt", "beta.txt", "gamma-rust.txt"]);
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
    let dir = fixture("sentence", &["hello_world.txt", "notes.txt"]);
    let config = Config {
        mode: Mode::SentenceSearch,
        query: "hello world".into(),
        path: dir.clone(),
    };

    assert_eq!(matched_names(&dir, &config), vec!["hello_world.txt"]);
    let _ = fs::remove_dir_all(&dir);
}
