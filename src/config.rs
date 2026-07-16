use std::path::PathBuf;

use crate::Args;

pub enum Mode {
    WordMatch,
    SentenceSearch,
    ContentSearch,
}

pub struct Config {
    pub mode: Mode,
    pub query: String,
    pub path: PathBuf,
}

impl Config {
    /// Build a `Config` from parsed CLI arguments.
    ///
    /// clap（`Args`）が唯一のパーサーで、ここではそれをドメインの
    /// `Config` に変換するだけ。排他・必須の検証は `Args` 側の
    /// `ArgGroup` で済んでいる前提だが、防御的にここでも確認する。
    pub fn from_args(args: Args) -> Result<Self, String> {
        let mode = if args.word_match {
            Mode::WordMatch
        } else if args.sentence {
            Mode::SentenceSearch
        } else if args.content {
            Mode::ContentSearch
        } else {
            return Err("検索モード（-m / -s / -c）を1つ指定してください。".to_string());
        };

        let query = args
            .query
            .ok_or_else(|| "検索するクエリを指定してください。".to_string())?;

        Ok(Config {
            mode,
            query,
            path: args.path,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(word_match: bool, sentence: bool, content: bool, query: Option<&str>) -> Args {
        Args {
            word_match,
            sentence,
            content,
            total_words: false,
            completions: false,
            query: query.map(str::to_string),
            path: PathBuf::from("."),
        }
    }

    #[test]
    fn from_args_maps_each_flag_to_mode() {
        assert!(matches!(
            Config::from_args(args(true, false, false, Some("q"))).unwrap().mode,
            Mode::WordMatch
        ));
        assert!(matches!(
            Config::from_args(args(false, true, false, Some("q"))).unwrap().mode,
            Mode::SentenceSearch
        ));
        assert!(matches!(
            Config::from_args(args(false, false, true, Some("q"))).unwrap().mode,
            Mode::ContentSearch
        ));
    }

    #[test]
    fn from_args_requires_a_mode() {
        assert!(Config::from_args(args(false, false, false, Some("q"))).is_err());
    }

    #[test]
    fn from_args_requires_a_query() {
        assert!(Config::from_args(args(true, false, false, None)).is_err());
    }
}