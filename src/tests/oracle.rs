//! Oracle harness: compare lazippier's output against the reference `7zz` CLI.

use std::process::Command;

fn which_7zz() -> Option<String> {
    for candidate in &[
        "/usr/bin/7zz",
        "/usr/local/bin/7zz",
        "/opt/homebrew/bin/7zz",
    ] {
        if std::path::Path::new(candidate).exists() {
            return Some((*candidate).to_owned());
        }
    }
    let out = Command::new("which").arg("7zz").output().ok()?;
    if out.status.success() {
        let path = String::from_utf8(out.stdout).ok()?.trim().to_owned();
        if !path.is_empty() {
            return Some(path);
        }
    }
    None
}

/// Skip the current test if `7zz` is not on PATH.
pub fn skip_if_no_7zz() -> bool {
    if which_7zz().is_none() {
        eprintln!("[skip] 7zz not found on PATH — oracle test skipped");
        return true;
    }
    false
}

/// Compress `input` with `7zz -m0=lzma2` and return the raw `.7z` archive bytes.
pub fn seven_zip_compress_lzma2(input: &[u8]) -> Vec<u8> {
    let sevenzip = which_7zz().expect("7zz not found");
    let dir = tempdir();
    let input_path = dir.join("input.bin");
    let archive_path = dir.join("output.7z");

    std::fs::write(&input_path, input).expect("write oracle input");

    let status = Command::new(&sevenzip)
        .args([
            "a",
            "-t7z",
            "-m0=lzma2",
            archive_path.to_str().unwrap(),
            input_path.to_str().unwrap(),
        ])
        .status()
        .expect("run 7zz a");

    assert!(status.success(), "7zz lzma2 compression failed: {status}");
    std::fs::read(&archive_path).expect("read oracle archive")
}

/// Decompress a `.7z` archive with `7zz` and return the first entry's bytes.
#[allow(dead_code)]
pub fn seven_zip_decompress(archive: &[u8]) -> Vec<u8> {
    let sevenzip = which_7zz().expect("7zz not found");
    let dir = tempdir();
    let archive_path = dir.join("input.7z");
    let out_dir = dir.join("out");
    std::fs::create_dir_all(&out_dir).expect("create out dir");

    std::fs::write(&archive_path, archive).expect("write oracle archive");

    let status = Command::new(&sevenzip)
        .args([
            "e",
            "-y",
            archive_path.to_str().unwrap(),
            &format!("-o{}", out_dir.display()),
        ])
        .status()
        .expect("run 7zz e");

    assert!(status.success(), "7zz extraction failed: {status}");

    let entry = std::fs::read_dir(&out_dir)
        .expect("read out dir")
        .filter_map(|e| e.ok())
        .find(|e| e.path().is_file())
        .unwrap_or_else(|| panic!("no output files after 7zz e"));

    std::fs::read(entry.path()).expect("read extracted file")
}

fn tempdir() -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("lazippier-oracle-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("create tempdir");
    dir
}
