//! Persistent record of the ORIGINAL values changed by each tweak, so that
//! everything Afina does can be undone — even after closing the app.
//!
//! Invariant: an entry is created the first time a tweak is turned on and is
//! never overwritten while it exists; it is removed once the originals are restored.

use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::Path;

use afina_platform::registry::{Hive, RawValue};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegBackup {
    pub hive: Hive,
    pub path: String,
    pub name: String,
    /// `None` means the value did not exist before Afina touched it.
    pub original: Option<RawValue>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PowerBackup {
    pub previous_scheme: String,
    pub created_scheme: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TweakBackup {
    #[serde(default)]
    pub registry: Vec<RegBackup>,
    #[serde(default)]
    pub power: Option<PowerBackup>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackupStore {
    #[serde(default)]
    pub tweaks: BTreeMap<String, TweakBackup>,
}

impl BackupStore {
    /// Loads the store. A missing file gives an empty store; a corrupt file is
    /// kept aside as `*.json.corrompido` instead of being silently lost.
    pub fn load_from(path: &Path) -> Self {
        let Ok(text) = fs::read_to_string(path) else {
            return Self::default();
        };
        serde_json::from_str(&text).unwrap_or_else(|_| {
            let _ = fs::rename(path, path.with_extension("json.corrompido"));
            Self::default()
        })
    }

    /// Atomic save: write a temp file, then rename it over the real one.
    pub fn save_to(&self, path: &Path) -> io::Result<()> {
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir)?;
        }
        let json = serde_json::to_string_pretty(self).map_err(io::Error::other)?;
        let tmp = path.with_extension("json.tmp");
        fs::write(&tmp, json)?;
        fs::rename(&tmp, path)
    }

    pub fn has(&self, id: &str) -> bool {
        self.tweaks.contains_key(id)
    }

    /// Ids of tweaks changed by Afina (i.e. with something to restore).
    pub fn changed_ids(&self) -> impl Iterator<Item = &str> {
        self.tweaks.keys().map(String::as_str)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn json_roundtrip_and_corrupt_file_is_preserved() {
        let dir = std::env::temp_dir().join(format!("afina_backup_test_{}", std::process::id()));
        let path = dir.join("backup.json");
        let _ = fs::remove_dir_all(&dir);

        assert_eq!(BackupStore::load_from(&path), BackupStore::default());

        let mut store = BackupStore::default();
        store.tweaks.insert(
            "game_mode".into(),
            TweakBackup {
                registry: vec![RegBackup {
                    hive: Hive::Hkcu,
                    path: r"Software\Microsoft\GameBar".into(),
                    name: "AutoGameModeEnabled".into(),
                    original: Some(RawValue::dword(0)),
                }],
                power: Some(PowerBackup { previous_scheme: "a".into(), created_scheme: "b".into() }),
            },
        );
        store.save_to(&path).unwrap();
        assert_eq!(BackupStore::load_from(&path), store);
        assert_eq!(store.changed_ids().collect::<Vec<_>>(), ["game_mode"]);

        fs::write(&path, "{ isto não é json").unwrap();
        assert_eq!(BackupStore::load_from(&path), BackupStore::default());
        assert!(path.with_extension("json.corrompido").exists(), "arquivo corrompido deve ser preservado");

        let _ = fs::remove_dir_all(&dir);
    }
}
