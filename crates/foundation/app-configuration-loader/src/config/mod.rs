//! Top-level [`AppConfig`] envelope and the version-migration seam.
//!
//! The on-disk document is:
//!
//! ```toml
//! version = 1
//!
//! theme_name = "Catppuccin Mocha"
//!
//! [logging]
//! level = "info"
//! enable_stdout = true
//! ```
//!
//! Breaking schema changes follow the termusic-style versioned config module
//! pattern: freeze [`crate::config::v1`], author a `v2` module, then extend
//! [`migrate_from_previous`] so old files upgrade without data loss.

pub mod v1;

use crate::paths::CURRENT_SCHEMA_VERSION;
use serde::{Deserialize, Serialize};
use v1::ConfigV1;

/// Current application configuration: the latest-version schema plus its
/// on-disk `version` tag. Loading older files migrates them forward into this
/// type before it is returned or saved.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppConfig {
    /// On-disk schema version tag; always [`CURRENT_SCHEMA_VERSION`] after a
    /// successful load/migration.
    pub version: u32,

    /// Body of the current schema version.
    #[serde(flatten)]
    pub body: ConfigV1,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            version: CURRENT_SCHEMA_VERSION,
            body: ConfigV1::with_defaults(),
        }
    }
}

impl AppConfig {
    /// Builds a configuration holding only documented defaults.
    pub fn with_defaults() -> Self {
        Self::default()
    }
}

/// Migrates a parsed previous-version body up to the current schema.
///
/// Identity for now (v1 *is* current); this function exists to prove the
/// migration seam end-to-end. When v2 lands, branch on `previous_version`
/// here (`1 => migrate_v2_from_v1(...)`) instead of editing the frozen v1.
///
/// # Panics (debug builds only)
///
/// Debug-asserts that callers pre-checked the version against
/// [`CURRENT_SCHEMA_VERSION`]; [`crate::load_at`] does this before calling.
pub fn migrate_from_previous(previous_version: u32, previous: ConfigV1) -> AppConfig {
    debug_assert!(
        previous_version <= CURRENT_SCHEMA_VERSION,
        "unsupported schema version {previous_version} reached migration"
    );
    AppConfig {
        version: CURRENT_SCHEMA_VERSION,
        body: previous,
    }
}
