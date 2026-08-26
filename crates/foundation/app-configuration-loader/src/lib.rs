//! Purpose: reads/writes app config (TOML), env overrides.
//!
//! This crate does NOT hold secrets — credentials live in os-keyring-secret-storage.
//!
//! # Design
//!
//! - **Precedence:** built-in defaults < on-disk TOML file < environment
//!   overrides (CLI flags and debounced live-writeback are wired in later
//!   phases on top of these same primitives).
//! - **Location:** `<OS config dir>/SunoMediaStation/config.toml`
//!   (macOS: `~/Library/Application Support/SunoMediaStation/config.toml`,
//!   Linux: `$XDG_CONFIG_HOME/SunoMediaStation/config.toml`).
//! - **Versioned schema modules** ([termusic prior-art pattern]):
//!   [`config::v1`] holds the v1 schema. Breaking changes add a `v2` module
//!   plus a step in [`migrate_from_previous`] instead of mutating v1, so old
//!   files survive upgrades without data loss. The on-disk file carries a
//!   top-level `version = <n>` tag.
//! - **Forward compatible:** unknown fields in the TOML are ignored (no
//!   `deny_unknown_fields`) so newer configs still open in older builds.
//!
//! [termusic prior-art pattern]: https://github.com/tramhao/termusic
//!
//! Typical entry points: [`load_or_create`] (boot-time: creates the file with
//! defaults if absent), [`load`], and [`AppConfig::save`] (atomic write).

mod config;
mod env;
mod error;
mod loader;
mod paths;

pub use config::v1::{ConfigV1, LoggingSettings};
pub use config::{AppConfig, migrate_from_previous};
pub use env::ENV_PREFIX;
pub use error::ConfigurationError;
pub use loader::{load, load_at, load_or_create, load_or_create_at};
pub use paths::{
    APP_DIR_NAME, CONFIG_FILE_NAME, CURRENT_SCHEMA_VERSION, config_dir, default_config_path,
};

use std::path::Path;

impl AppConfig {
    /// Saves this configuration to its default OS location atomically.
    ///
    /// See [`paths::default_config_path`] for the exact location.
    pub fn save(&self) -> Result<(), ConfigurationError> {
        self.save_at(&default_config_path()?)
    }

    /// Saves this configuration to `path` atomically (temp file + rename).
    pub fn save_at(&self, path: &Path) -> Result<(), ConfigurationError> {
        loader::save_atomic(self, path)
    }
}
