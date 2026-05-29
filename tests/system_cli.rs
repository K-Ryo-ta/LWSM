use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn fixture(names: &[&str]) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("lwsm_l_{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    for name in names {
        fs::File::create(dir.join(name)).unwrap();
    }
    dir
}

#[test]
fn cli_m_prints_only_word_matched_files() {
    let dir = fixture(&["readme.md", "notes.txt"]);
    let output = Command::new(env!("CARGO_BIN_EXE_lwsm"))
        .args(["-m", "readme", dir.to_str().unwrap()])
        .output()
        .expect("failed to run lwsm");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("readme.md"));
    assert!(!stdout.contains("notes.txt"));

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn cli_s_prints_sentence_matched_files() {
    let dir = fixture(&["hello_world.txt", "other.txt"]);
    let output = Command::new(env!("CARGO_BIN_EXE_lwsm"))
        .args(["-s", "hello world", dir.to_str().unwrap()])
        .output()
        .expect("failed to run lwsm");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("hello_world.txt"));
    assert!(!stdout.contains("other.txt"));

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn cli_fails_on_unknown_flag() {
    let output = Command::new(env!("CARGO_BIN_EXE_lwsm"))
        .args(["-x", "query", "."])
        .output()
        .unwrap();

    assert!(!output.status.success());
}
