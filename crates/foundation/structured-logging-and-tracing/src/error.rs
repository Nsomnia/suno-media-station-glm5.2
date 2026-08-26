//! The single error type returned by logging setup.

use std::path::PathBuf;
use thiserror::Error;

/// Everything that can go wrong while installing structured logging.
#[derive(Debug, Error)]
pub enum LoggingSetupError {
    /// The OS-local data directory could not be located, so there is nowhere
    /// sensible to place log files.
    #[error("could not locate the OS-local data directory for log storage")]
    DataDirectoryUnavailable,

    /// The resolved log directory could not be created (permissions, disk, …).
    #[error("failed to create log directory {path:?}")]
    LogDirectoryCreation {
        /// Directory that could not be created.
        path: PathBuf,
        /// Underlying filesystem error.
        #[source]
        source: std::io::Error,
    },

    /// The rotating file appender could not be initialized inside the
    /// (already-created) log directory.
    #[error("failed to open rotating log file in {directory:?}")]
    LogFileInitialization {
        /// Directory the appender was pointed at.
        directory: PathBuf,
        /// Underlying appender initialization error.
        #[source]
        source: tracing_appender::rolling::InitError,
    },

    /// A filter directive (either `RUST_LOG` or the settings default) failed
    /// to parse as an [`tracing_subscriber::EnvFilter`] directive string.
    #[error("invalid log filter directive {directive:?}")]
    InvalidFilterDirective {
        /// The directive string that failed to parse.
        directive: String,
        /// Underlying parse error.
        #[source]
        source: tracing_subscriber::filter::ParseError,
    },

    /// The global subscriber could not be installed — typically because
    /// another global default subscriber was already set.
    #[error("failed to install global tracing subscriber")]
    SubscriberInstallation {
        /// Underlying installation error.
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}
