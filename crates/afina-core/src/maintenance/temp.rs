//! Temporary files cleanup. Only files older than 24 h; files in use are skipped;
//! symlinks are never followed. Shader caches are deliberately NOT touched.

use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use crate::Result;

const MAX_AGE: Duration = Duration::from_secs(24 * 3600);

/// Deletes files older than `max_age` under `dir` (recursively) and removes
/// sub-folders left empty. Returns the number of bytes freed.
pub fn clean_dir(dir: &Path, max_age: Duration) -> u64 {
    let Ok(entries) = fs::read_dir(dir) else { return 0 };
    let now = SystemTime::now();
    let mut freed = 0;
    for e in entries.flatten() {
        let path = e.path();
        let Ok(meta) = fs::symlink_metadata(&path) else { continue };
        if meta.file_type().is_symlink() {
            continue; // never follow links out of the temp folder
        }
        if meta.is_dir() {
            freed += clean_dir(&path, max_age);
            let _ = fs::remove_dir(&path); // succeeds only when empty
        } else {
            let old_enough =
                meta.modified().ok().and_then(|m| now.duration_since(m).ok()).is_some_and(|age| age >= max_age);
            if old_enough && fs::remove_file(&path).is_ok() {
                freed += meta.len();
            }
        }
    }
    freed
}

pub(super) fn clean_temp() -> Result<String> {
    let mut dirs: Vec<PathBuf> = Vec::new();
    if let Some(t) = std::env::var_os("TEMP") {
        dirs.push(t.into());
    }
    if let Some(w) = std::env::var_os("SystemRoot") {
        dirs.push(PathBuf::from(w).join("Temp"));
    }
    let freed: u64 = dirs.iter().map(|d| clean_dir(d, MAX_AGE)).sum();
    Ok(format!("{:.0} MB liberados (arquivos em uso ou com menos de 24 h foram mantidos).", freed as f64 / 1_048_576.0))
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn only_removes_old_enough_files() {
        let dir = std::env::temp_dir().join(format!("afina_clean_test_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("sub")).unwrap();
        fs::write(dir.join("novo.txt"), "x").unwrap();
        fs::write(dir.join("sub").join("novo2.txt"), "y").unwrap();

        assert_eq!(clean_dir(&dir, Duration::from_secs(3600)), 0, "nada tem 1 hora ainda");
        assert!(dir.join("novo.txt").exists());

        assert_eq!(clean_dir(&dir, Duration::ZERO), 2);
        assert!(!dir.join("sub").exists(), "subpasta vazia deve ser removida");
        let _ = fs::remove_dir_all(&dir);
    }
}
