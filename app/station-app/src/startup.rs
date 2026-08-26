//! Startup wiring: config → logging → initial theme → shell state.
//!
//! Everything here is composition only; behavior lives in the dependency
//! crates. Failure policy: config problems fall back to documented defaults
//! (a desktop app should still open), logging setup failure is fatal because
//! every later phase assumes it.

use std::sync::{Arc, Mutex};

use app_configuration_loader::AppConfig;
use design_tokens_theme_definitions::DesignTokens;
use structured_logging_and_tracing::{LoggingGuard, LoggingSettings};
use ui_app_shell_and_navigation::ShellState;

/// Everything `main` needs from a successful startup sequence.
pub struct Bootstrapped {
    /// Installed tracing subscriber guard — keep alive for the process.
    pub logging_guard: LoggingGuard,
    /// The shell, pre-wired with theme persistence.
    pub shell_state: ShellState,
}

impl Bootstrapped {
    /// Runs the full startup sequence in order.
    #[must_use]
    pub fn run() -> Self {
        let config = Arc::new(Mutex::new(load_config_with_fallback()));
        let logging_guard = install_logging(&lock_config_with_fallback(&config).body.logging);
        let initial_theme =
            resolve_initial_theme(&lock_config_with_fallback(&config).body.theme_name);

        let mut shell_state = ShellState::new(initial_theme);
        let persist_sink = Arc::clone(&config);
        shell_state.set_theme_change_handler(Box::new(move |theme_name| {
            persist_theme_choice(&persist_sink, theme_name);
        }));

        Self {
            logging_guard,
            shell_state,
        }
    }
}

/// Loads the OS-located config file, creating it with defaults if absent.
///
/// Any load error (corrupt file, unsupported future schema) degrades to
/// defaults with a startup warning rather than blocking launch.
fn load_config_with_fallback() -> AppConfig {
    match app_configuration_loader::load_or_create() {
        Ok(config) => config,
        Err(error) => {
            // Logging is not installed yet, so this can only go to stderr.
            eprintln!("station-app: config load failed ({error}); using defaults");
            AppConfig::with_defaults()
        }
    }
}

/// Reads a snapshot from the shared config without panicking on poison.
///
/// Same failure policy as [`persist_theme_choice`]: no `.expect()` on the
/// config mutex. A poisoned mutex means another thread panicked while
/// holding it, but the data itself is still readable — we log a warning and
/// recover the inner value so boot proceeds. Boot-time access is
/// single-threaded, so poisoning here is effectively unreachable; the
/// handling exists for uniformity with the persistence path.
fn lock_config_with_fallback(shared: &Arc<Mutex<AppConfig>>) -> AppConfig {
    match shared.lock() {
        Ok(config) => config.clone(),
        Err(poisoned) => {
            tracing::warn!("config mutex poisoned at boot; recovering inner value");
            poisoned.into_inner().clone()
        }
    }
}

fn install_logging(settings: &app_configuration_loader::LoggingSettings) -> LoggingGuard {
    let mapped = LoggingSettings {
        default_filter_directive: settings.level.clone(),
        stdout_enabled: settings.enable_stdout,
        ..LoggingSettings::default()
    };
    structured_logging_and_tracing::initialize_structured_logging(&mapped)
        .expect("structured logging setup failed")
}

/// Resolves the configured theme name against the shipped registry,
/// falling back to the default theme when the name is unknown (e.g. an
/// edited config file).
#[must_use]
pub fn resolve_initial_theme(theme_name: &str) -> DesignTokens {
    match design_tokens_theme_definitions::theme_by_name(theme_name) {
        Some(tokens) => tokens,
        None => {
            tracing::warn!(configured = %theme_name, "unknown theme name; using default");
            design_tokens_theme_definitions::default_theme()
        }
    }
}

/// Persists a theme switch by rewriting the shared config atomically
/// (direct save; debouncing deferred per Phase 0 scope).
fn persist_theme_choice(shared: &Arc<Mutex<AppConfig>>, theme_name: &str) {
    let result = shared
        .lock()
        .map(|mut config| {
            config.body.theme_name = theme_name.to_owned();
            config.save().map_err(|error| error.to_string())
        })
        .unwrap_or_else(|_| Err("config mutex poisoned".to_owned()));
    if let Err(error) = result {
        tracing::error!(%error, "failed to persist theme choice to config file");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_theme_name_resolves_to_that_theme() {
        let tokens = resolve_initial_theme("Catppuccin Latte");
        assert_eq!(tokens.name, "Catppuccin Latte");
        assert!(!tokens.is_dark);
    }

    #[test]
    fn unknown_theme_name_falls_back_to_default() {
        let tokens = resolve_initial_theme("Definitely Not Shipped");
        assert_eq!(
            tokens.name,
            design_tokens_theme_definitions::default_theme().name
        );
    }
}
