//! User-tunable settings that control how structured logging is set up.
//!
//! Deliberately independent of `app-configuration-loader`: that crate may later
//! map its parsed configuration onto [`LoggingSettings`], but this crate must
//! not depend on it (layering: both live in `foundation`, siblings never depend
//! on siblings).

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// How often the rotating log file rolls over to a fresh file.
///
/// Serialized in lowercase form (`"daily"`) so TOML config stays readable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogRotationPeriod {
    /// A new log file every day at midnight local time.
    #[default]
    Daily,
    /// A new log file every hour.
    Hourly,
    /// A new log file every minute (debugging only — very chatty on disk).
    Minutely,
    /// One log file for the whole process lifetime, suffixed with the start date.
    Never,
}

impl LogRotationPeriod {
    /// Maps this setting onto the equivalent [`tracing_appender::rolling::Rotation`].
    #[must_use]
    pub fn tracing_rotation(self) -> tracing_appender::rolling::Rotation {
        match self {
            Self::Daily => tracing_appender::rolling::Rotation::DAILY,
            Self::Hourly => tracing_appender::rolling::Rotation::HOURLY,
            Self::Minutely => tracing_appender::rolling::Rotation::MINUTELY,
            Self::Never => tracing_appender::rolling::Rotation::NEVER,
        }
    }
}

/// Settings controlling structured logging setup.
///
/// Construct via [`LoggingSettings::default`] and adjust individual fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoggingSettings {
    /// `EnvFilter` directive string used when `RUST_LOG` is unset or empty.
    ///
    /// Defaults to `"info"`; accepts any valid `EnvFilter` directive syntax
    /// (e.g. `"info,suno_http_client_core=debug"`).
    pub default_filter_directive: String,

    /// Explicit directory that rotating log files are written into.
    ///
    /// When `None`, logs go beneath the OS-appropriate local data directory
    /// (see [`crate::directory_resolution`]).
    pub log_directory_override: Option<PathBuf>,

    /// How often the log file rotates. Defaults to [`LogRotationPeriod::Daily`].
    pub rotation_period: LogRotationPeriod,

    /// Whether the human-readable stdout layer is installed alongside the file
    /// layer. Defaults to `true`.
    pub stdout_enabled: bool,
}

impl Default for LoggingSettings {
    fn default() -> Self {
        Self {
            default_filter_directive: String::from("info"),
            log_directory_override: None,
            rotation_period: LogRotationPeriod::default(),
            stdout_enabled: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_sane() {
        let settings = LoggingSettings::default();
        assert_eq!(settings.default_filter_directive, "info");
        assert_eq!(settings.log_directory_override, None);
        assert_eq!(settings.rotation_period, LogRotationPeriod::Daily);
        assert!(settings.stdout_enabled);
    }

    #[test]
    fn rotation_period_serializes_in_lowercase_and_round_trips() {
        for period in [
            LogRotationPeriod::Daily,
            LogRotationPeriod::Hourly,
            LogRotationPeriod::Minutely,
            LogRotationPeriod::Never,
        ] {
            let serialized = serde_json::to_string(&period).expect("serialize");
            assert_eq!(
                serialized.to_lowercase(),
                serialized,
                "{serialized} should already be lowercase"
            );
            let deserialized: LogRotationPeriod =
                serde_json::from_str(&serialized).expect("deserialize");
            assert_eq!(deserialized, period);
        }
    }

    #[test]
    fn rotation_period_maps_onto_tracing_rotations() {
        assert_eq!(
            format!("{:?}", LogRotationPeriod::Daily.tracing_rotation()),
            "Rotation(Daily)"
        );
        assert_eq!(
            format!("{:?}", LogRotationPeriod::Hourly.tracing_rotation()),
            "Rotation(Hourly)"
        );
        assert_eq!(
            format!("{:?}", LogRotationPeriod::Minutely.tracing_rotation()),
            "Rotation(Minutely)"
        );
        // `NEVER` wraps its unit variant in the same tuple struct.
        assert_eq!(
            format!("{:?}", LogRotationPeriod::Never.tracing_rotation()),
            "Rotation(Never)"
        );
    }
}
