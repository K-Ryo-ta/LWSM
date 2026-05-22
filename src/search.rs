use std::fs::DirEntry;

use crate::config::{Config, Mode};

pub fn filter_entries(entries: Vec<DirEntry>, config: &Config) -> Vec<DirEntry> {
    entries
        .into_iter()
        .filter(|entry| {
            let file_name = entry.file_name();
            let name = file_name.to_string_lossy();

            is_match(&name, &config.query, &config.mode)
        })
        .collect()
}

fn is_match(file_name: &str, query: &str, mode: &Mode) -> bool {
    match mode {
        Mode::WordMatch => word_match(file_name, query),
        Mode::SentenceSearch => sentence_search(file_name, query),
    }
}

fn word_match(file_name: &str, query: &str) -> bool {
    let normalized_name = normalize(file_name);
    let normalized_query = normalize(query);

    normalized_name
        .split_whitespace()
        .any(|word| word == normalized_query)
}

fn sentence_search(file_name: &str, query: &str) -> bool {
    let normalized_name = normalize(file_name);
    let normalized_query = normalize(query);

    normalized_name.contains(&normalized_query)
}

fn normalize(text: &str) -> String {
    text.to_lowercase()
        .replace(['_', '-', '.', '/', '\\'], " ")
}