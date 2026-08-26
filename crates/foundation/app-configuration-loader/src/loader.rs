//! Load / create / save orchestration over a TOML config document.

use crate::config::v1::ConfigV1;
use crate::config::{AppConfig, migrate_from_previous};
use crate::error::ConfigurationError;
use crate::paths::{CURRENT_SCHEMA_VERSION, default_config_path};
use serde::Deserialize;
use std::fs;
use std::path::Path;

/// Header comment written when the file is first created. Defaults themselves
/// are always serialized from the struct so file and defaults cannot drift.
const CREATED_FILE_HEADER: &str = "\
# Suno Media Station configuration.
#
# Generated with default values; safe to edit while the app is not running
# (the app atomically rewrites this file on settings changes). Environment
# variables prefixed with `SMS_` override these values at runtime.
";

/// Loads the configuration from its default OS location, creating directories
/// and the file itself (with documented defaults) if they do not exist yet,
/// then applies environment overrides on top.
pub fn load_or_create() -> Result<AppConfig, ConfigurationError> {
    load_or_create_at(&default_config_path()?)
}

/// Like [`load_or_create`] but against an explicit path (used by tests and
/// future multi-profile support).
pub fn load_or_create_at(path: &Path) -> Result<AppConfig, ConfigurationError> {
    if path.exists() {
        return load_at(path);
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|source| ConfigurationError::Io {
            source,
            path: parent.to_path_buf(),
        })?;
    }
    let config = AppConfig::with_defaults();
    write_atomic(&config, path)?;
    Ok(config)
}

/// Loads the configuration from its default OS location; errors if absent.
/// Environment overrides are applied after the file values.
pub fn load() -> Result<AppConfig, ConfigurationError> {
    load_at(&default_config_path()?)
}

/// Like [`load`] but against an explicit path.
///
/// Precedence: built-in defaults < file < environment. Files tagged with an
/// older supported version are migrated forward via
/// [`migrate_from_previous`]; newer versions are rejected descriptively.
pub fn load_at(path: &Path) -> Result<AppConfig, ConfigurationError> {
    let raw = fs::read_to_string(path).map_err(|source| ConfigurationError::Io {
        source,
        path: path.to_path_buf(),
    })?;

    // Probe only the version tag first so unknown future versions fail fast
    // instead of half-deserializing into the current schema.
    let probe: VersionProbe = parse_toml(&raw, path)?;
    let mut config = match probe.version {
        version if version <= CURRENT_SCHEMA_VERSION => {
            let body: ConfigV1 = parse_toml(&raw, path)?;
            migrate_from_previous(version, body)
        }
        found => {
            return Err(ConfigurationError::UnsupportedSchemaVersion {
                found,
                max_supported: CURRENT_SCHEMA_VERSION,
            });
        }
    };
    config.apply_env_overrides_from_process()?;
    Ok(config)
}

/// Serializes `config` and replaces `path` atomically: write to a sibling
/// temp file, then rename it over the target (atomic within a filesystem).
pub(crate) fn save_atomic(config: &AppConfig, path: &Path) -> Result<(), ConfigurationError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|source| ConfigurationError::Io {
            source,
            path: parent.to_path_buf(),
        })?;
    }
    write_atomic(config, path)
}

fn write_atomic(config: &AppConfig, path: &Path) -> Result<(), ConfigurationError> {
    let body = toml::to_string_pretty(config)?;
    let temp = path.with_extension("toml.tmp");
    fs::write(&temp, format!("{CREATED_FILE_HEADER}{body}")).map_err(|source| {
        ConfigurationError::Io {
            source,
            path: temp.clone(),
        }
    })?;
    fs::rename(&temp, path).map_err(|source| ConfigurationError::Io {
        source,
        path: path.to_path_buf(),
    })?;
    Ok(())
}

fn parse_toml<T: for<'de> Deserialize<'de>>(
    raw: &str,
    path: &Path,
) -> Result<T, ConfigurationError> {
    toml::from_str(raw).map_err(|source| ConfigurationError::MalformedToml {
        source,
        path: path.to_path_buf(),
    })
}

#[derive(Deserialize)]
struct VersionProbe {
    #[serde(default = "missing_version_is_v1")]
    version: u32,
}

fn missing_version_is_v1() -> u32 {
    1
}
