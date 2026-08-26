//! Resolution of the OS-appropriate configuration directory and file path.

use crate::error::ConfigurationError;
use std::path::PathBuf;

/// Directory name for this app inside the OS config dir.
///
/// Deliberately CamelCase without spaces: keeps paths shell-friendly and
/// matches how many desktop apps name their settings folders.
pub const APP_DIR_NAME: &str = "SunoMediaStation";

/// File name of the configuration document inside [`config_dir`].
pub const CONFIG_FILE_NAME: &str = "config.toml";

/// Highest config schema version this build understands; see
/// [`crate::migrate_from_previous`].
pub const CURRENT_SCHEMA_VERSION: u32 = 1;

/// Returns `<OS config dir>/SunoMediaStation`, e.g.
/// `~/Library/Application Support/SunoMediaStation` on macOS or
/// `$XDG_CONFIG_HOME/SunoMediaStation` on Linux.
pub fn config_dir() -> Result<PathBuf, ConfigurationError> {
    let base = dirs::config_dir().ok_or(ConfigurationError::NoConfigDir)?;
    Ok(base.join(APP_DIR_NAME))
}

/// Returns the full default configuration file path:
/// `[config_dir]/config.toml`.
pub fn default_config_path() -> Result<PathBuf, ConfigurationError> {
    Ok(config_dir()?.join(CONFIG_FILE_NAME))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_path_is_config_dir_plus_file_name() {
        // Only meaningful when a config dir exists (CI runners provide one);
        // skips rather than fails on exotic environments without one.
        let Ok(dir) = config_dir() else {
            return;
        };
        assert!(dir.ends_with(APP_DIR_NAME));
        assert_eq!(
            default_config_path().expect("path"),
            dir.join(CONFIG_FILE_NAME)
        );
    }
}
