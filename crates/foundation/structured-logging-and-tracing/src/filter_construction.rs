//! Construction of the shared [`EnvFilter`] that drives every layer.
//!
//! Honors `RUST_LOG` first, falling back to the settings-provided default
//! directive when the variable is unset or empty.

use crate::error::LoggingSetupError;
use crate::settings::LoggingSettings;
use tracing_subscriber::EnvFilter;

/// Standard tracing verbosity environment variable.
const RUST_LOG_VARIABLE: &str = "RUST_LOG";

/// Builds the env filter from the live process environment.
///
/// Thin wrapper over [`build_env_filter_from`] reading `RUST_LOG`; kept
/// separate so tests can exercise the pure logic without mutating process
/// state from parallel test threads.
pub(crate) fn build_env_filter(settings: &LoggingSettings) -> Result<EnvFilter, LoggingSetupError> {
    let raw_override = std::env::var(RUST_LOG_VARIABLE).ok();
    build_env_filter_from(raw_override.as_deref(), settings)
}

/// Builds the env filter from an explicit `RUST_LOG` value (or `None`).
///
/// An empty or whitespace-only override counts as "unset" and falls back to
/// [`LoggingSettings::default_filter_directive`].
pub(crate) fn build_env_filter_from(
    raw_rust_log_override: Option<&str>,
    settings: &LoggingSettings,
) -> Result<EnvFilter, LoggingSetupError> {
    let effective_directive = raw_rust_log_override
        .map(str::trim)
        .filter(|raw| !raw.is_empty())
        .unwrap_or(&settings.default_filter_directive);

    EnvFilter::try_new(effective_directive).map_err(|source| {
        LoggingSetupError::InvalidFilterDirective {
            directive: effective_directive.to_owned(),
            source,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn settings_with_default(directive: &str) -> LoggingSettings {
        LoggingSettings {
            default_filter_directive: directive.to_owned(),
            ..LoggingSettings::default()
        }
    }

    #[test]
    fn unset_rust_log_falls_back_to_settings_default() {
        let filter = build_env_filter_from(None, &settings_with_default("warn"))
            .expect("filter should build");
        assert_eq!(
            filter.to_string(),
            "warn",
            "fallback directive should be used verbatim"
        );
    }

    #[test]
    fn empty_and_whitespace_rust_log_count_as_unset() {
        for raw_override in [Some(""), Some("   ")] {
            let filter = build_env_filter_from(raw_override, &settings_with_default("debug"))
                .expect("filter should build");
            assert_eq!(filter.to_string(), "debug");
        }
    }

    #[test]
    fn rust_log_override_wins_over_settings_default() {
        let filter = build_env_filter_from(
            Some("info,structured_logging_and_tracing=trace"),
            &settings_with_default("error"),
        )
        .expect("filter should build");
        assert!(
            filter
                .to_string()
                .contains("structured_logging_and_tracing=trace")
        );
    }

    #[test]
    fn invalid_directive_is_reported_with_its_text() {
        // A bare `=` with no target or level is not a valid directive.
        let error = build_env_filter_from(Some("="), &settings_with_default("info"))
            .expect_err("invalid directive should fail");
        match error {
            LoggingSetupError::InvalidFilterDirective { directive, .. } => {
                assert_eq!(directive, "=");
            }
            other => panic!("expected InvalidFilterDirective, got {other:?}"),
        }
    }
}
