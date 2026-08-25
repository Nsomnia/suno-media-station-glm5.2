//! `check-layering`: scans every workspace crate's Cargo.toml dependency
//! tables and fails if any crate depends on a *higher* layer than its own.
//!
//! Layers (lower number = lower level):
//!   0 crates/foundation · 1 crates/external-bridges · 2 crates/domain-stores
//!   3 crates/application-services · 4 crates/ui
//!
//! `crates/shared-test-support` and the `app/station-app` composition root are
//! layer-free by design: anything may depend on them, and they may depend on
//! anything. Such edges are reported as informational notes, never silently.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// Result of one full check pass.
pub fn run() -> Result<(), ()> {
    let repo_root = repo_root();
    let mut members: Vec<Member> = Vec::new();

    let groups = [
        ("crates/foundation", 0),
        ("crates/external-bridges", 1),
        ("crates/domain-stores", 2),
        ("crates/application-services", 3),
        ("crates/ui", 4),
        ("crates/shared-test-support", LAYER_FREE),
    ];
    for (group_dir, layer) in groups {
        let dir = repo_root.join(group_dir);
        let entries = std::fs::read_dir(&dir)
            .unwrap_or_else(|e| panic!("check-layering: cannot read {}: {e}", dir.display()));
        for entry in entries.flatten() {
            let manifest = entry.path().join("Cargo.toml");
            if !manifest.is_file() {
                continue;
            }
            let pkg_name = parse_package_name(&manifest).unwrap_or_else(|| {
                panic!("check-layering: no package name in {}", manifest.display())
            });
            members.push(Member {
                name: pkg_name,
                manifest,
                layer: Some(layer),
            });
        }
    }

    // The composition root may depend on any layer.
    let app_manifest = repo_root.join("app/station-app/Cargo.toml");
    if app_manifest.is_file() {
        let pkg_name = parse_package_name(&app_manifest).unwrap_or_else(|| {
            panic!(
                "check-layering: no package name in {}",
                app_manifest.display()
            )
        });
        members.push(Member {
            name: pkg_name,
            manifest: app_manifest,
            layer: None,
        });
    }

    let layer_of_name: BTreeMap<String, Option<i32>> =
        members.iter().map(|m| (m.name.clone(), m.layer)).collect();

    let mut violations: Vec<String> = Vec::new();
    let mut notes: Vec<String> = Vec::new();
    let mut edges = 0usize;

    for m in &members {
        for dep in parse_dependency_names(&m.manifest) {
            let Some(&dep_layer) = layer_of_name.get(&dep) else {
                continue; // external (non-workspace) dependency
            };
            edges += 1;
            match (m.layer, dep_layer) {
                // Both layered: forbid upward edges only.
                (Some(from), Some(to)) if to > from => violations.push(format!(
                    "LAYER VIOLATION: `{}` (layer {from}) depends on `{}` (layer {to})\n  at {}",
                    m.name,
                    dep,
                    m.manifest.display()
                )),
                // Anything -> shared-test-support / station-app, or from those: allowed by design.
                _ => notes.push(format!(
                    "note: layer-free edge `{}` -> `{}` (allowed by design)",
                    m.name, dep
                )),
            }
        }
    }

    println!(
        "check-layering: scanned {} crates, {edges} intra-workspace edges",
        members.len()
    );
    for n in &notes {
        println!("{n}");
    }
    if violations.is_empty() {
        println!("check-layering: OK");
        Ok(())
    } else {
        eprintln!();
        for v in &violations {
            eprintln!("{v}");
        }
        eprintln!(
            "\ncheck-layering: FAILED with {} violation(s)",
            violations.len()
        );
        Err(())
    }
}

const LAYER_FREE: i32 = -1;

struct Member {
    name: String,
    manifest: PathBuf,
    layer: Option<i32>,
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .expect("xtask lives directly under the repo root")
        .to_path_buf()
}

fn parse_package_name(manifest: &Path) -> Option<String> {
    let text = std::fs::read_to_string(manifest).ok()?;
    let mut in_package = false;
    for line in text.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            in_package = line == "[package]";
            continue;
        }
        if !in_package || line.starts_with('#') || line.is_empty() {
            continue;
        }
        if let Some(value) = line.strip_prefix("name")?.trim_start().strip_prefix('=') {
            return Some(value.trim().trim_matches('"').to_string());
        }
    }
    None
}

/// Collects dependency names declared under `[dependencies]`,
/// `[dev-dependencies]`, and `[build-dependencies]`.
///
/// Deliberately simple text scanning (per task spec): it reads the token left
/// of the first `=` inside those tables. Renamed deps (`foo = { package =
/// "bar" }`) resolve to the local alias `foo`, which matches our internal
/// naming convention anyway.
fn parse_dependency_names(manifest: &Path) -> Vec<String> {
    let text = std::fs::read_to_string(manifest).unwrap_or_default();
    const DEP_TABLES: [&str; 3] = [
        "[dependencies]",
        "[dev-dependencies]",
        "[build-dependencies]",
    ];
    let mut names = Vec::new();
    let mut in_dep_table = false;
    for raw in text.lines() {
        let line = strip_comment(raw).trim().to_string();
        if line.starts_with('[') {
            in_dep_table = DEP_TABLES.contains(&line.as_str());
            continue;
        }
        if !in_dep_table || line.is_empty() {
            continue;
        }
        if let Some((key, _)) = line.split_once('=') {
            let key = key.trim().trim_matches('"');
            if !key.is_empty() {
                names.push(key.to_string());
            }
        }
    }
    names.sort();
    names.dedup();
    names
}

fn strip_comment(line: &str) -> &str {
    match line.find(" #") {
        Some(idx) => &line[..idx],
        None => line,
    }
}
