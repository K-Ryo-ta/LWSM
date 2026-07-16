use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

// テストは並列実行されるため、テストごとに一意な一時ディレクトリを使う。
// プロセスIDだけでは同一プロセス内の別テストと衝突するので、
// アトミックカウンタを組み合わせて衝突を避ける。
fn unique_dir(label: &str) -> PathBuf {
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!("lwsm_{label}_{}_{n}", std::process::id()))
}

fn fixture(label: &str, names: &[&str]) -> PathBuf {
    let dir = unique_dir(label);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    for name in names {
        fs::File::create(dir.join(name)).unwrap();
    }
    dir
}

#[test]
fn cli_m_prints_only_word_matched_files() {
    let dir = fixture("m", &["readme.md", "notes.txt"]);
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
    let dir = fixture("s", &["hello_world.txt", "other.txt"]);
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
fn cli_c_prints_only_content_matched_files() {
    let dir = unique_dir("c");
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("hit.txt"), "this file has the keyword TODO inside").unwrap();
    fs::write(dir.join("miss.txt"), "nothing relevant here").unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_lwsm"))
        .args(["-c", "todo", dir.to_str().unwrap()])
        .output()
        .expect("failed to run lwsm");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("hit.txt"));
    assert!(!stdout.contains("miss.txt"));

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

#[test]
fn cli_fails_when_multiple_modes_given() {
    let output = Command::new(env!("CARGO_BIN_EXE_lwsm"))
        .args(["-m", "-s", "query", "."])
        .output()
        .unwrap();

    assert!(!output.status.success());
}

#[test]
fn cli_fails_when_no_mode_given() {
    let output = Command::new(env!("CARGO_BIN_EXE_lwsm"))
        .args(["query", "."])
        .output()
        .unwrap();

    assert!(!output.status.success());
}
