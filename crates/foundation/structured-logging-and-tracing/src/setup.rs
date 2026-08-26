//! Installation of the process-global [`tracing`] subscriber.
//!
//! Composes a shared [`EnvFilter`] with (optionally) a human-readable stdout
//! layer and always-on JSON file layer written through a non-blocking
//! rotating file appender.

use crate::directory_resolution as directory;
use crate::error::LoggingSetupError;
use crate::filter_construction as filter;
use crate::settings::LoggingSettings;
use std::path::Path;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::RollingFileAppender;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

/// Keeps the non-blocking log writer alive and flushes buffered events on drop.
///
/// # Why the caller must keep this alive
///
/// Log events are written by a background worker thread; dropping
/// [`LoggingGuard`] shuts that worker down, flushing any still-buffered events
/// first. If the guard is dropped (or never bound) immediately after setup,
/// log output is lost or truncated. Bind it at the top level, e.g. in `main`.
#[derive(Debug)]
pub struct LoggingGuard {
    _file_writer_guard: WorkerGuard,
}

/// Installs structured logging using the live process environment for the
/// OS data directory.
pub fn initialize_structured_logging(
    settings: &LoggingSettings,
) -> Result<LoggingGuard, LoggingSetupError> {
    initialize_structured_logging_with_base_data_directory(settings, None)
}

/// Installs structured logging with an injected base data directory.
///
/// The injection point exists so tests (and unusual embedding hosts) can pin
/// where logs land without touching real user directories.
pub fn initialize_structured_logging_with_base_data_directory(
    settings: &LoggingSettings,
    base_data_directory: Option<&Path>,
) -> Result<LoggingGuard, LoggingSetupError> {
    let log_directory = directory::resolve_log_directory(
        settings.log_directory_override.as_deref(),
        base_data_directory,
    )?;
    directory::ensure_directory_exists(&log_directory)?;
    let env_filter = filter::build_env_filter(settings)?;

    let rolling_file_appender = RollingFileAppender::builder()
        .rotation(settings.rotation_period.tracing_rotation())
        .filename_prefix(directory::LOG_FILE_NAME_PREFIX)
        .filename_suffix(directory::LOG_FILE_NAME_SUFFIX)
        .build(&log_directory)
        .map_err(|source| LoggingSetupError::LogFileInitialization {
            directory: log_directory.clone(),
            source,
        })?;
    let (non_blocking_file_writer, worker_guard) =
        tracing_appender::non_blocking(rolling_file_appender);

    let stdout_layer = settings.stdout_enabled.then(|| {
        tracing_subscriber::fmt::layer()
            .pretty()
            .with_writer(std::io::stdout)
    });
    let json_file_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_ansi(false)
        .with_writer(non_blocking_file_writer);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(stdout_layer)
        .with(json_file_layer)
        .try_init()
        .map_err(|source| LoggingSetupError::SubscriberInstallation {
            source: Box::new(source),
        })?;

    // Setup diagnostics: the only events this crate emits itself.
    tracing::info!(
        directory = %log_directory.display(),
        rotation = ?settings.rotation_period,
        "structured logging initialized"
    );

    Ok(LoggingGuard {
        _file_writer_guard: worker_guard,
    })
}
