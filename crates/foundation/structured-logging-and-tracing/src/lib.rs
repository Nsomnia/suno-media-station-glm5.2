//! Purpose: tracing subscriber setup, log file rotation.
//!
//! This crate installs the process-global [`tracing`] subscriber used by every
//! other crate in the workspace. It wires together:
//!
//! - a human-readable stdout layer (optional, enabled by default), and
//! - a JSON file layer written through [`tracing_appender::rolling`] with
//!   configurable rotation into the OS-appropriate data/log directory, made
//!   non-blocking via [`tracing_appender::non_blocking`].
//!
//! This crate does NOT emit log events itself beyond its own setup diagnostics.
//!
//! # Usage
//!
//! ```no_run
//! use structured_logging_and_tracing::{initialize_structured_logging, LoggingSettings};
//!
//! let settings = LoggingSettings::default();
//! // The caller must keep the returned guard alive for the lifetime of the
//! // process: dropping it shuts down the non-blocking log writer, flushing any
//! // buffered events before the writer thread exits.
//! let _logging_guard = initialize_structured_logging(&settings)
//!     .expect("structured logging setup failed");
//! ```
//!
//! Log verbosity is controlled by the standard `RUST_LOG` environment variable;
//! when it is unset or empty, [`LoggingSettings::default_filter_directive`]
//! (default `"info"`) is used instead.

pub mod directory_resolution;
pub mod error;
pub mod filter_construction;
pub mod settings;
pub mod setup;

pub use directory_resolution::{
    APPLICATION_DATA_DIRECTORY_NAME, LOG_FILE_NAME_PREFIX, LOG_FILE_NAME_SUFFIX,
    LOGS_DIRECTORY_NAME,
};
pub use error::LoggingSetupError;
pub use settings::{LogRotationPeriod, LoggingSettings};
pub use setup::{
    LoggingGuard, initialize_structured_logging,
    initialize_structured_logging_with_base_data_directory,
};
