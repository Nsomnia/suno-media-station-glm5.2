//! Resolution of the directory that rotating log files are written into.
//!
//! Resolution is injectable for testability: callers may supply an explicit
//! base data directory instead of letting this module query the real OS
//! directories via the `dirs` crate.

use crate::error::LoggingSetupError;
use std::path::{Path, PathBuf};

/// Name of the application-specific subdirectory under the OS local data dir.
pub const APPLICATION_DATA_DIRECTORY_NAME: &str = "suno-media-station";

/// Name of the logs subdirectory inside the application data dir.
pub const LOGS_DIRECTORY_NAME: &str = "logs";

/// Filename prefix for rotating log files.
pub const LOG_FILE_NAME_PREFIX: &str = "station-app";

/// Filename suffix (extension) for rotating log files.
pub const LOG_FILE_NAME_SUFFIX: &str = "log";

/// Resolves the directory rotating log files are written into.
///
/// - If `directory_override` is set, it wins verbatim.
/// - Otherwise `<base>/suno-media-station/logs` is used, where `base` comes
///   from the injected `base_data_directory` when provided, or the OS-local
///   data directory (`dirs::data_local_dir`) otherwise.
///
/// Returns [`LoggingSetupError::DataDirectoryUnavailable`] only when no
/// override and no base directory are available from any source.
pub(crate) fn resolve_log_directory(
    directory_override: Option<&Path>,
    base_data_directory: Option<&Path>,
) -> Result<PathBuf, LoggingSetupError> {
    resolve_from_parts(
        directory_override,
        base_data_directory.map(Path::to_path_buf),
        dirs::data_local_dir(),
    )
}

/// Pure core of [`resolve_log_directory`] with the OS directory injected,
/// keeping the "no directory anywhere" branch deterministic under test.
fn resolve_from_parts(
    directory_override: Option<&Path>,
    base_data_directory: Option<PathBuf>,
    os_local_data_directory: Option<PathBuf>,
) -> Result<PathBuf, LoggingSetupError> {
    if let Some(override_path) = directory_override {
        return Ok(override_path.to_path_buf());
    }

    let base = base_data_directory
        .or(os_local_data_directory)
        .ok_or(LoggingSetupError::DataDirectoryUnavailable)?;

    Ok(base
        .join(APPLICATION_DATA_DIRECTORY_NAME)
        .join(LOGS_DIRECTORY_NAME))
}

/// Creates the log directory (and parents) if it does not already exist.
pub(crate) fn ensure_directory_exists(directory: &Path) -> Result<(), LoggingSetupError> {
    std::fs::create_dir_all(directory).map_err(|source| LoggingSetupError::LogDirectoryCreation {
        path: directory.to_path_buf(),
        source,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn override_wins_verbatim() {
        let resolved = resolve_log_directory(
            Some(Path::new("/custom/log/place")),
            Some(Path::new("/injected/base")),
        )
        .expect("override path should always win");
        assert_eq!(resolved, PathBuf::from("/custom/log/place"));
    }

    #[test]
    fn injected_base_data_directory_is_joined_deterministically() {
        let resolved = resolve_log_directory(None, Some(Path::new("/injected/base/data")))
            .expect("injected base should resolve without touching the OS");
        assert_eq!(
            resolved,
            PathBuf::from("/injected/base/data")
                .join(APPLICATION_DATA_DIRECTORY_NAME)
                .join(LOGS_DIRECTORY_NAME)
        );
    }

    #[test]
    fn missing_base_and_os_data_dir_reports_unavailable() {
        let result = resolve_from_parts(None, None, None);
        assert!(matches!(
            result,
            Err(LoggingSetupError::DataDirectoryUnavailable)
        ));
    }

    #[test]
    fn injected_base_takes_priority_over_os_data_dir() {
        let resolved = resolve_from_parts(
            None,
            Some(PathBuf::from("/injected/base/data")),
            Some(PathBuf::from("/os/data/dir")),
        )
        .expect("injected base should resolve");
        let expected = PathBuf::from("/injected/base/data")
            .join(APPLICATION_DATA_DIRECTORY_NAME)
            .join(LOGS_DIRECTORY_NAME);
        assert_eq!(resolved, expected);
    }
}
