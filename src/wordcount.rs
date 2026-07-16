use std::fs;
use std::io;
use std::path::Path;

/// 1ファイルの文字数（空白・改行・タブを除く）を数える。
pub fn count_file(path: &Path) -> Result<usize, io::Error> {
    let contents = fs::read_to_string(path)?;
    Ok(count_chars(&contents))
}

/// 指定パスの文字数を集計して出力する。
///
/// ファイルならそのファイルの文字数を、ディレクトリなら直下の各
/// ファイルの文字数と合計を出力する。読み取れないファイル
/// （バイナリ等）はスキップする。
pub fn print_word_counts(path: &Path) -> Result<(), io::Error> {
    if path.is_file() {
        let count = count_file(path)?;
        println!("{:>8}  {}", count, path.display());
        return Ok(());
    }

    let mut entries = fs::read_dir(path)?.collect::<Result<Vec<_>, io::Error>>()?;
    entries.sort_by_key(|entry| entry.file_name());

    let mut total = 0usize;
    for entry in entries {
        let entry_path = entry.path();
        if !entry_path.is_file() {
            continue;
        }

        match count_file(&entry_path) {
            Ok(count) => {
                total += count;
                println!("{:>8}  {}", count, entry.file_name().to_string_lossy());
            }
            // バイナリなど UTF-8 として読めないファイルはスキップ
            Err(_) => continue,
        }
    }

    println!("{:>8}  total", total);
    Ok(())
}

fn count_chars(contents: &str) -> usize {
    contents.chars().filter(|c| !c.is_whitespace()).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_chars_excludes_whitespace() {
        assert_eq!(count_chars("hello world"), 10);
        assert_eq!(count_chars("  hello   world  \n foo\tbar "), 16);
        assert_eq!(count_chars(""), 0);
        assert_eq!(count_chars("   \n  \t "), 0);
    }

    #[test]
    fn count_chars_counts_multibyte_characters() {
        assert_eq!(count_chars("今日はいい天気"), 7);
        assert_eq!(count_chars("あ い\tう\nえ お"), 5);
    }

    #[test]
    fn count_file_reads_and_counts() {
        let dir = std::env::temp_dir().join(format!("lwsm_wc_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let file = dir.join("sample.txt");
        fs::write(&file, "one two three four").unwrap();

        assert_eq!(count_file(&file).unwrap(), 15);
        let _ = fs::remove_dir_all(&dir);
    }
}
