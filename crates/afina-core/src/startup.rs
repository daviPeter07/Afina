//! Startup apps. Uses the same `StartupApproved` mechanism as Task Manager's
//! "Startup apps" tab, so changes show up (and can be undone) there too.
//! Nothing is ever deleted.

use std::path::PathBuf;

use afina_platform::registry::{self, Hive, RawValue};

use crate::{Error, Result};

const APPROVED: &str = r"Software\Microsoft\Windows\CurrentVersion\Explorer\StartupApproved";

/// Entries never offered for disabling.
const PROTECTED: &[&str] = &["SecurityHealth", "WindowsDefender"];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Source {
    UserRun,
    MachineRun,
    MachineRun32,
    UserFolder,
    CommonFolder,
}

impl Source {
    fn approved_key(self) -> (Hive, String) {
        match self {
            Source::UserRun => (Hive::Hkcu, format!(r"{APPROVED}\Run")),
            Source::MachineRun => (Hive::Hklm, format!(r"{APPROVED}\Run")),
            Source::MachineRun32 => (Hive::Hklm, format!(r"{APPROVED}\Run32")),
            Source::UserFolder => (Hive::Hkcu, format!(r"{APPROVED}\StartupFolder")),
            Source::CommonFolder => (Hive::Hklm, format!(r"{APPROVED}\StartupFolder")),
        }
    }

    pub fn needs_admin(self) -> bool {
        !matches!(self, Source::UserRun | Source::UserFolder)
    }

    /// Who is affected, for display.
    pub fn scope_label(self) -> &'static str {
        if self.needs_admin() { "todos os usuários" } else { "só você" }
    }
}

#[derive(Clone, Debug)]
pub struct StartupItem {
    pub name: String,
    pub command: String,
    pub source: Source,
    pub enabled: bool,
}

/// Task Manager format: first byte even = enabled (0x02), odd = disabled (0x03).
pub fn approved_bytes_enabled(bytes: Option<&[u8]>) -> bool {
    match bytes {
        Some([first, ..]) => first & 1 == 0,
        _ => true,
    }
}

fn is_enabled(source: Source, name: &str) -> bool {
    let (hive, path) = source.approved_key();
    let raw = registry::read_opt(hive, &path, name);
    approved_bytes_enabled(raw.as_ref().map(|r| r.data.as_slice()))
}

/// Every startup entry (Run keys + Startup folders), sorted by name.
pub fn list() -> Vec<StartupItem> {
    let mut items = Vec::new();
    let run_keys = [
        (Source::UserRun, Hive::Hkcu, r"Software\Microsoft\Windows\CurrentVersion\Run"),
        (Source::MachineRun, Hive::Hklm, r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run"),
        (Source::MachineRun32, Hive::Hklm, r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Run"),
    ];
    for (source, hive, path) in run_keys {
        for (name, value) in registry::list_values(hive, path) {
            if name.is_empty() || PROTECTED.iter().any(|p| name.eq_ignore_ascii_case(p)) {
                continue;
            }
            let enabled = is_enabled(source, &name);
            items.push(StartupItem { command: value.as_string().unwrap_or_default(), name, source, enabled });
        }
    }
    let folders = [(Source::UserFolder, "APPDATA"), (Source::CommonFolder, "ProgramData")];
    for (source, env) in folders {
        let Some(base) = std::env::var_os(env) else { continue };
        let dir = PathBuf::from(base).join(r"Microsoft\Windows\Start Menu\Programs\Startup");
        let Ok(entries) = std::fs::read_dir(&dir) else { continue };
        for e in entries.flatten() {
            let name = e.file_name().to_string_lossy().into_owned();
            if name.eq_ignore_ascii_case("desktop.ini") {
                continue;
            }
            let enabled = is_enabled(source, &name);
            items.push(StartupItem { command: e.path().display().to_string(), name, source, enabled });
        }
    }
    items.sort_by_key(|i| i.name.to_lowercase());
    items
}

pub fn set_enabled(item: &StartupItem, enabled: bool) -> Result<()> {
    let (hive, path) = item.source.approved_key();
    let mut bytes = [0u8; 12];
    bytes[0] = if enabled { 0x02 } else { 0x03 };
    registry::write(hive, &path, &item.name, &RawValue::binary(&bytes)).map_err(|e| Error::io(item.name.clone(), e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn approved_flag_parsing() {
        assert!(approved_bytes_enabled(None));
        assert!(approved_bytes_enabled(Some(&[])));
        assert!(approved_bytes_enabled(Some(&[0x02, 0, 0])));
        assert!(approved_bytes_enabled(Some(&[0x06, 0, 0])));
        assert!(!approved_bytes_enabled(Some(&[0x03, 0xAA, 0xBB])));
        assert!(!approved_bytes_enabled(Some(&[0x01])));
    }

    #[test]
    fn listing_hides_protected_entries() {
        assert!(list().iter().all(|i| !i.name.eq_ignore_ascii_case("SecurityHealth")));
    }
}
