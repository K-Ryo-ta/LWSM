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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_lowercases_and_splits_separators() {
        assert_eq!(normalize("Hello-World.txt"), "hello world txt");
        assert_eq!(normalize("foo_bar"), "foo bar");
    }

    #[test]
    fn word_match_requires_whole_word() {
        assert!(word_match("readme.md", "readme"));
        assert!(!word_match("readme.md", "read"));
    }

    #[test]
    fn word_match_splits_on_hyphens() {
        assert!(word_match("my-rust-lib", "rust"));
    }

    #[test]
    fn sentence_search_matches_substring() {
        assert!(sentence_search("readme.md", "read"));
        assert!(sentence_search("hello_world.txt", "hello world"));
    }

    #[test]
    fn sentence_search_is_case_insensitive() {
        assert!(sentence_search("README.md", "readme"));
    }
}