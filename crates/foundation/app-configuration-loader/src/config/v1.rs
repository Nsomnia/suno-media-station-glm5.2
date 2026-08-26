//! The v1 configuration schema.
//!
//! This module is **frozen** once shipped: breaking changes must add a `v2`
//! sibling module and a migration step instead of editing these types
//! (termusic's versioned config modules prior-art pattern). Purely additive
//! optional fields may land here while v1 is current.

use serde::{Deserialize, Serialize};

/// Schema version 1 body — the fields written alongside the top-level
/// `version` tag in the TOML document.
///
/// Unknown fields are intentionally tolerated (no `deny_unknown_fields`) so
/// configs written by newer builds still load here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfigV1 {
    /// Name of the visual theme to apply at startup.
    #[serde(default = "default_theme_name")]
    pub theme_name: String,

    /// Logging behaviour.
    #[serde(default)]
    pub logging: LoggingSettings,
}

impl Default for ConfigV1 {
    fn default() -> Self {
        Self {
            theme_name: default_theme_name(),
            logging: LoggingSettings::default(),
        }
    }
}

fn default_theme_name() -> String {
    "Catppuccin Mocha".to_string()
}

impl ConfigV1 {
    /// Builds the documented defaults for schema version 1.
    pub fn with_defaults() -> Self {
        Self::default()
    }
}

/// Logging configuration section (`[logging]` in the TOML file).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoggingSettings {
    /// Log filter level as a plain string (e.g. `"info"`, `"debug"`).
    /// Parsed into a real filter by structured-logging-and-tracing.
    #[serde(default = "default_log_level")]
    pub level: String,

    /// Whether logs are also mirrored to stdout/stderr.
    #[serde(default = "default_enable_stdout")]
    pub enable_stdout: bool,
}

impl Default for LoggingSettings {
    fn default() -> Self {
        Self {
            level: default_log_level(),
            enable_stdout: default_enable_stdout(),
        }
    }
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_enable_stdout() -> bool {
    true
}
