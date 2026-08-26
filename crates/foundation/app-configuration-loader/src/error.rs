//! Error type for configuration loading, parsing, and persistence.

use std::path::PathBuf;

/// Everything that can go wrong while resolving, reading, or writing the
/// application configuration file.
#[derive(Debug, thiserror::Error)]
pub enum ConfigurationError {
    /// The OS did not report a user config directory (e.g. `$HOME` unset).
    #[error("no OS config directory is available for this platform/user")]
    NoConfigDir,

    /// An I/O failure while creating directories, reading, or writing files.
    #[error("I/O error on `{path}`: {source}")]
    Io {
        /// Underlying OS error.
        #[source]
        source: std::io::Error,
        /// File or directory the operation targeted.
        path: PathBuf,
    },

    /// The config file exists but is not valid TOML.
    #[error("config file `{path}` is not valid TOML: {source}")]
    MalformedToml {
        /// Underlying TOML parse error (includes line/column context).
        #[source]
        source: toml::de::Error,
        /// Config file that failed to parse.
        path: PathBuf,
    },

    /// The file's `version` tag is newer than this build supports.
    #[error(
        "config schema version {found} is not supported by this build \
         (max supported: {max_supported}); upgrade the application"
    )]
    UnsupportedSchemaVersion {
        /// Version tag found in the file.
        found: u32,
        /// Highest version this build can migrate from.
        max_supported: u32,
    },

    /// The configuration could not be serialized back to TOML.
    #[error("failed to serialize configuration to TOML: {0}")]
    TomlSerialize(#[from] toml::ser::Error),

    /// An environment override was present but could not be parsed.
    #[error("environment override `{variable}` = `{value}` is invalid: {expected}")]
    InvalidEnvOverride {
        /// Environment variable name (e.g. `SMS_LOGGING_ENABLE_STDOUT`).
        variable: String,
        /// Raw value as read from the environment.
        value: String,
        /// Human-readable description of the expected format.
        expected: &'static str,
    },
}
