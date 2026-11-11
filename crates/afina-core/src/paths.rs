//! Where Afina keeps its files: `%LOCALAPPDATA%\Afina`.

use std::path::PathBuf;

pub fn data_dir() -> PathBuf {
    let base = std::env::var_os("LOCALAPPDATA").map(PathBuf::from).unwrap_or_else(|| PathBuf::from("."));
    base.join("Afina")
}

/// Original values of every change (see [`crate::backup`]).
pub fn backup_file() -> PathBuf {
    data_dir().join("backup.json")
}

/// Human-readable activity log.
pub fn log_file() -> PathBuf {
    data_dir().join("afina.log")
}
