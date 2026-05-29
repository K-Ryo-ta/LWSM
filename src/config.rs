use std::env;
use std::path::PathBuf;

pub enum Mode {
    WordMatch,
    SentenceSearch,
}

pub struct Config {
    pub mode: Mode,
    pub query: String,
    pub path: PathBuf,
}

pub fn parse_args() -> Result<Config, String> {
    let mut args = env::args().skip(1);

    let mode_arg = args.next().ok_or_else(usage)?;
    let query = args.next().ok_or_else(usage)?;

    let path = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));

    if args.next().is_some() {
        return Err(format!("引数が多すぎます。\n\n{}", usage()));
    }

    let mode = parse_mode(&mode_arg)?;

    Ok(Config { mode, query, path })
}

fn parse_mode(mode_arg: &str) -> Result<Mode, String> {
    match mode_arg {
        "-m" | "--match" => Ok(Mode::WordMatch),
        "-s" | "--sentence" => Ok(Mode::SentenceSearch),
        _ => Err(usage()),
    }
}

fn usage() -> String {
    String::from(
        "Usage:
  lwsm -m <word> [path]
  lwsm -s <sentence> [path]

Examples:
  lwsm -m rust
  lwsm -s \"hello world\"
  lwsm -m test ./src",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_mode_accepts_short_and_long_flags() {
        assert!(matches!(parse_mode("-m").unwrap(), Mode::WordMatch));
        assert!(matches!(parse_mode("--match").unwrap(), Mode::WordMatch));
        assert!(matches!(parse_mode("-s").unwrap(), Mode::SentenceSearch));
        assert!(matches!(parse_mode("--sentence").unwrap(), Mode::SentenceSearch));
    }

    #[test]
    fn parse_mode_rejects_unknown_flag() {
        assert!(parse_mode("-x").is_err());
        assert!(parse_mode("").is_err());
    }
}