pub mod config;
pub mod entries;
pub mod output;
pub mod search;
mod gencomp;

use clap::Parser;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "lwsm", about = "list & search files by word/sentence")]
pub struct Args {
    #[arg(short = 'm', long = "match", help = "word match search")]
    pub word_match: bool,

    #[arg(short = 's', long = "sentence", help = "sentence search")]
    pub sentence: bool,

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

    let config = config::parse_args()
        .map_err(|msg| io::Error::new(io::ErrorKind::InvalidInput, msg))?;

    let entries = entries::read_entries(&config.path)?;
    let matched_entries = search::filter_entries(entries, &config);

    output::print_entries(matched_entries)?;

    Ok(())
}