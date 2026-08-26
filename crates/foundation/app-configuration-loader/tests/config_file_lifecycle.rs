//! End-to-end tests over real files in a tempdir, covering the documented
//! lifecycle guarantees: creation-with-defaults, round-trip persistence,
//! version tagging/migration, and malformed-input handling.

use app_configuration_loader::{AppConfig, ConfigV1, load_or_create_at};
use std::fs;
use std::path::PathBuf;

/// Isolated config file inside its own tempdir (auto-cleaned on drop).
struct TestFile {
    _dir: tempfile::TempDir,
    path: PathBuf,
}

impl TestFile {
    fn new() -> Self {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("SunoMediaStation").join("config.toml");
        Self { _dir: dir, path }
    }
}

fn write_raw(file: &TestFile, contents: &str) {
    fs::create_dir_all(file.path.parent().expect("parent")).expect("mkdir");
    fs::write(&file.path, contents).expect("write fixture");
}

fn read_raw(file: &TestFile) -> String {
    fs::read_to_string(&file.path).expect("read back")
}

#[test]
fn missing_file_is_created_with_documented_defaults() {
    let file = TestFile::new();
    let loaded = load_or_create_at(&file.path).expect("create + load");

    assert_eq!(loaded, AppConfig::with_defaults());
    assert_eq!(loaded.version, 1);
    assert_eq!(loaded.body.theme_name, "Catppuccin Mocha");
    assert_eq!(loaded.body.logging.level, "info");
    assert!(loaded.body.logging.enable_stdout);

    let on_disk = read_raw(&file);
    assert!(on_disk.contains("version = 1"), "version tag written");
    assert!(on_disk.contains("Catppuccin Mocha"));
    assert!(on_disk.starts_with('#'), "header comment present");
}

#[test]
fn save_and_load_round_trip_preserves_edits() {
    let file = TestFile::new();
    let mut config = load_or_create_at(&file.path).expect("bootstrap");

    config.body.theme_name = "Custom Theme".to_string();
    config.body.logging.level = "debug".to_string();
    config.body.logging.enable_stdout = false;
    config.save_at(&file.path).expect("save");

    let reloaded = load_or_create_at(&file.path).expect("reload");
    assert_eq!(reloaded, config);

    // Atomic writes must not leave temp files behind.
    let siblings: Vec<_> = fs::read_dir(file.path.parent().expect("parent"))
        .expect("dir listing")
        .map(|entry| entry.expect("entry").file_name())
        .collect();
    assert!(
        siblings.iter().all(|name| name != "config.toml.tmp"),
        "no temp residue: {siblings:?}"
    );
}

#[test]
fn file_without_version_tag_loads_as_v1_and_gains_tag_on_save() {
    let file = TestFile::new();
    write_raw(
        &file,
        "theme_name = \"Legacy\"\n\n[logging]\nlevel = \"warn\"\nenable_stdout = true\n",
    );

    let migrated = load_or_create_at(&file.path).expect("migration is identity for v1");
    assert_eq!(migrated.version, 1);
    assert_eq!(migrated.body.theme_name, "Legacy");

    migrated.save_at(&file.path).expect("resave");
    assert!(read_raw(&file).contains("version = 1"), "tag materialized");
}

#[test]
fn future_schema_version_is_rejected_descriptively() {
    let file = TestFile::new();
    write_raw(&file, "version = 2\ntheme_name = \"From The Future\"\n");

    let error = load_or_create_at(&file.path).expect_err("must reject v2");
    assert!(
        error.to_string().contains("schema version 2"),
        "descriptive message, got: {error}"
    );
}

#[test]
fn malformed_toml_is_an_error_not_a_panic() {
    let file = TestFile::new();
    write_raw(&file, "theme_name = [unterminated");

    let error = load_or_create_at(&file.path).expect_err("must fail cleanly");
    assert!(
        error.to_string().contains("not valid TOML"),
        "descriptive message, got: {error}"
    );
}

#[test]
fn unknown_fields_are_tolerated_for_forward_compat() {
    let file = TestFile::new();
    write_raw(
        &file,
        "version = 1\ntheme_name = \"Kept\"\nsome_future_field = true\n",
    );

    let loaded = load_or_create_at(&file.path).expect("unknown fields ignored");
    assert_eq!(
        loaded.body,
        ConfigV1 {
            theme_name: "Kept".to_string(),
            logging: Default::default(),
        }
    );
}
