pub mod config;
pub mod entries;
pub mod output;
pub mod search;
pub mod wordcount;
mod gencomp;

use clap::{ArgGroup, Parser};
use std::io;
use std::path::{Path, PathBuf};

use crate::config::Config;

#[derive(Parser)]
#[command(
    name = "lwsm",
    about = "list & search files by word / sentence / content"
)]
#[command(group(
    ArgGroup::new("mode")
        .args(["word_match", "sentence", "content"])
        .multiple(false)
        .required(false)
))]
pub struct Args {
    #[arg(short = 'm', long = "match", help = "word match search on names")]
    pub word_match: bool,

    #[arg(short = 's', long = "sentence", help = "sentence search on names")]
    pub sentence: bool,

    #[arg(short = 'c', long = "content", help = "search file contents by keyword")]
    pub content: bool,

    #[arg(
        short = 't',
        long = "total-words",
        help = "count characters in a file or all files in a directory"
    )]
    pub total_words: bool,

    #[arg(long, help = "generate completion files", default_value_t = false)]
    pub completions: bool,

    pub query: Option<String>,

    #[arg(default_value = ".")]
    pub path: PathBuf,
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.completions {
        gencomp::generate(Path::new("completions"));
        return Ok(());
    }

    if args.total_words {
        // -t はクエリを取らないので、最初の位置引数をパスとして扱う。
        // `lwsm -t ./file.txt` と `lwsm -t` (カレントディレクトリ) の
        // どちらも自然に動くようにする。
        let target = args
            .query
            .map(PathBuf::from)
            .unwrap_or_else(|| args.path.clone());
        wordcount::print_word_counts(&target)?;
        return Ok(());
    }

    let config = Config::from_args(args)
        .map_err(|msg| io::Error::new(io::ErrorKind::InvalidInput, msg))?;

    let entries = entries::read_entries(&config.path)?;
    let matched_entries = search::filter_entries(entries, &config);

    output::print_entries(matched_entries)?;

    Ok(())
}