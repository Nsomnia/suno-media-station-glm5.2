//! `check-file-caps`: walks `crates/`, `app/`, and `xtask/` and enforces the
//! repo's Rust file-size caps (docs/02, doc 18):
//!   >300 lines  -> HARD CAP, check fails
//!   >200 lines  -> soft cap, warning printed
//!
//! Fail-safe: if a `.rs` file cannot be read, the check fails with a clear
//! message rather than skipping it silently.

use std::path::{Path, PathBuf};

const WARN_LINES: usize = 200;
const FAIL_LINES: usize = 300;
const WALK_ROOTS: [&str; 3] = ["crates", "app", "xtask"];

pub fn run() -> Result<(), ()> {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .expect("xtask lives directly under the repo root");

    let mut files: Vec<(PathBuf, usize)> = Vec::new();
    for root in WALK_ROOTS {
        let dir = repo_root.join(root);
        if !dir.is_dir() {
            // Missing tree is suspicious (repo layout regression) — fail loud.
            eprintln!(
                "check-file-caps: expected directory missing: {}",
                dir.display()
            );
            return Err(());
        }
        collect_rs_files(&dir, &mut files);
    }

    files.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    let mut hard_offenders: Vec<(&PathBuf, usize)> = Vec::new();
    println!("check-file-caps: scanned {} .rs file(s)", files.len());
    for (path, lines) in &files {
        if *lines > FAIL_LINES {
            hard_offenders.push((path, *lines));
        } else if *lines > WARN_LINES {
            println!(
                "WARN (>{} lines): {} ({lines} lines)",
                WARN_LINES,
                path.display()
            );
        }
    }

    if hard_offenders.is_empty() {
        println!("check-file-caps: OK (hard cap = {FAIL_LINES} lines)");
        Ok(())
    } else {
        eprintln!();
        for (path, lines) in &hard_offenders {
            eprintln!(
                "HARD CAP VIOLATION (>{} lines): {} ({lines} lines) — split into submodules",
                FAIL_LINES,
                path.display()
            );
        }
        eprintln!(
            "\ncheck-file-caps: FAILED with {} file(s) over the hard cap",
            hard_offenders.len()
        );
        Err(())
    }
}

fn collect_rs_files(dir: &Path, out: &mut Vec<(PathBuf, usize)>) {
    let entries = std::fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("check-file-caps: cannot read {}: {e}", dir.display()));
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_rs_files(&path, out);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            let text = std::fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("check-file-caps: cannot read {}: {e}", path.display()));
            let lines = text.lines().count();
            out.push((path, lines));
        }
    }
}
