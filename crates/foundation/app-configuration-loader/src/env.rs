//! Environment-variable overrides applied after the TOML file is read.
//!
//! Convention: variables prefixed with [`ENV_PREFIX`] override the
//! corresponding file value, giving the documented precedence
//! *defaults < file < env* (CLI and live-writeback come later on top).
//!
//! Mapping (flat, section path in SCREAMING_SNAKE_CASE):
//!
//! | Variable                    | Overrides              |
//! |-----------------------------|------------------------|
//! | `SMS_THEME_NAME`            | `body.theme_name`      |
//! | `SMS_LOGGING_LEVEL`         | `body.logging.level`   |
//! | `SMS_LOGGING_ENABLE_STDOUT` | `body.logging.enable_stdout` |

use crate::config::AppConfig;
use crate::error::ConfigurationError;

/// Prefix for all environment overrides (`Suno Media Station Settings`).
pub const ENV_PREFIX: &str = "SMS_";

impl AppConfig {
    /// Applies overrides by looking up variable names via `lookup`.
    ///
    /// `lookup` is injectable so callers/tests can substitute their own
    /// environment source instead of `std::env::var`.
    pub fn apply_env_overrides(
        &mut self,
        lookup: &dyn Fn(&str) -> Option<String>,
    ) -> Result<(), ConfigurationError> {
        if let Some(value) = lookup(&format!("{ENV_PREFIX}THEME_NAME")) {
            self.body.theme_name = value;
        }
        if let Some(value) = lookup(&format!("{ENV_PREFIX}LOGGING_LEVEL")) {
            self.body.logging.level = value;
        }
        if let Some(raw) = lookup(&format!("{ENV_PREFIX}LOGGING_ENABLE_STDOUT")) {
            let parsed =
                raw.trim()
                    .parse::<bool>()
                    .map_err(|_| ConfigurationError::InvalidEnvOverride {
                        variable: format!("{ENV_PREFIX}LOGGING_ENABLE_STDOUT"),
                        expected: "`true` or `false`",
                        value: raw.clone(),
                    })?;
            self.body.logging.enable_stdout = parsed;
        }
        Ok(())
    }

    /// Applies overrides from the real process environment.
    pub fn apply_env_overrides_from_process(&mut self) -> Result<(), ConfigurationError> {
        self.apply_env_overrides(&|name| std::env::var(name).ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fake_env<'a>(pairs: &'a [(&'a str, &'a str)]) -> impl Fn(&str) -> Option<String> + 'a {
        move |name: &str| {
            pairs
                .iter()
                .find(|(key, _)| format!("{ENV_PREFIX}{key}") == name)
                .map(|(_, value)| (*value).to_string())
        }
    }

    #[test]
    fn overrides_replace_file_values() {
        let mut config = AppConfig::with_defaults();
        config
            .apply_env_overrides(&fake_env(&[
                ("THEME_NAME", "Catppuccin Latte"),
                ("LOGGING_LEVEL", "debug"),
                ("LOGGING_ENABLE_STDOUT", "false"),
            ]))
            .expect("overrides apply");
        assert_eq!(config.body.theme_name, "Catppuccin Latte");
        assert_eq!(config.body.logging.level, "debug");
        assert!(!config.body.logging.enable_stdout);
    }

    #[test]
    fn unset_variables_leave_values_untouched() {
        let expected = AppConfig::with_defaults();
        let mut config = AppConfig::with_defaults();
        config.apply_env_overrides(&fake_env(&[])).expect("no-op");
        assert_eq!(config, expected);
    }

    #[test]
    fn malformed_bool_is_a_descriptive_error_not_a_panic() {
        let mut config = AppConfig::with_defaults();
        let error = config
            .apply_env_overrides(&fake_env(&[("LOGGING_ENABLE_STDOUT", "yes-please")]))
            .expect_err("must fail");
        let ConfigurationError::InvalidEnvOverride { variable, .. } = error else {
            panic!("expected InvalidEnvOverride, got {error:?}");
        };
        assert_eq!(variable, "SMS_LOGGING_ENABLE_STDOUT");
    }
}
