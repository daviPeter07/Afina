//! [`Kind::DxGlobalFlag`](crate::tweak::Kind::DxGlobalFlag): one `key=value;` entry inside
//! `HKCU\...\DirectX\UserGpuPreferences\DirectXUserGlobalSettings`.
//!
//! Windows' own Graphics settings page writes other keys into the same string,
//! so we only ever touch OUR key and preserve everything else.

use afina_platform::registry::{self, Hive, RawValue};

use crate::backup::{BackupStore, RegBackup, TweakBackup};
use crate::{Error, Result};

const PATH: &str = r"Software\Microsoft\DirectX\UserGpuPreferences";
const NAME: &str = "DirectXUserGlobalSettings";

/// Value of `key` inside a `k=v;` list.
pub fn get_kv(list: &str, key: &str) -> Option<String> {
    list.split(';').filter_map(|p| p.split_once('=')).find(|(k, _)| k.trim() == key).map(|(_, v)| v.trim().to_string())
}

/// Sets (`Some`) or removes (`None`) `key`, keeping every other entry intact and in order.
pub fn set_kv(list: &str, key: &str, value: Option<&str>) -> String {
    let mut out = String::new();
    let mut written = false;
    for part in list.split(';').map(str::trim).filter(|p| !p.is_empty()) {
        let k = part.split_once('=').map_or(part, |(k, _)| k.trim());
        if k != key {
            out.push_str(part);
            out.push(';');
        } else if let (Some(v), false) = (value, written) {
            out.push_str(&format!("{key}={v};"));
            written = true;
        }
    }
    if let (Some(v), false) = (value, written) {
        out.push_str(&format!("{key}={v};"));
    }
    out
}

fn current() -> String {
    registry::read_string(Hive::Hkcu, PATH, NAME).unwrap_or_default()
}

fn write_flag(key: &str, value: Option<&str>) -> Result<()> {
    let new = set_kv(&current(), key, value);
    let raw = (!new.is_empty()).then(|| RawValue::string(&new));
    registry::put(Hive::Hkcu, PATH, NAME, raw.as_ref()).map_err(|e| Error::io(NAME, e))
}

/// Our key's value in the snapshot taken before the first change.
fn original_flag(key: &str, backup: &TweakBackup) -> Option<String> {
    let original = backup.registry.first()?.original.as_ref()?.as_string()?;
    get_kv(&original, key)
}

pub(super) fn is_on(key: &str) -> bool {
    get_kv(&current(), key).as_deref() == Some("1")
}

pub(super) fn turn_on(id: &str, key: &str, store: &mut BackupStore) -> Result<()> {
    if !store.has(id) {
        let original = registry::read(Hive::Hkcu, PATH, NAME).map_err(|e| Error::io(NAME, e))?;
        let reg = RegBackup { hive: Hive::Hkcu, path: PATH.into(), name: NAME.into(), original };
        store.tweaks.insert(id.into(), TweakBackup { registry: vec![reg], power: None });
    }
    write_flag(key, Some("1"))
}

pub(super) fn turn_off(id: &str, key: &str, store: &mut BackupStore) -> Result<()> {
    if let Some(b) = store.tweaks.remove(id) {
        write_flag(key, original_flag(key, &b).as_deref())?;
    }
    if is_on(key) {
        write_flag(key, Some("0"))?;
    }
    Ok(())
}

pub(super) fn restore(key: &str, backup: &TweakBackup) -> Result<()> {
    write_flag(key, original_flag(key, backup).as_deref())
}

#[cfg(test)]
mod tests {
    use super::*;

    const K: &str = "SwapEffectUpgradeEnable";

    #[test]
    fn set_kv_adds_replaces_and_removes_only_our_key() {
        assert_eq!(set_kv("", K, Some("1")), "SwapEffectUpgradeEnable=1;");
        assert_eq!(set_kv("VRROptimizeEnable=0;", K, Some("1")), "VRROptimizeEnable=0;SwapEffectUpgradeEnable=1;");
        assert_eq!(
            set_kv("SwapEffectUpgradeEnable=0;AutoHDREnable=1;", K, Some("1")),
            "SwapEffectUpgradeEnable=1;AutoHDREnable=1;"
        );
        assert_eq!(set_kv("A=1;SwapEffectUpgradeEnable=1;B=2", K, None), "A=1;B=2;");
    }

    #[test]
    fn get_kv_reads_value() {
        assert_eq!(get_kv("A=1;SwapEffectUpgradeEnable=1;", K).as_deref(), Some("1"));
        assert_eq!(get_kv("A=1;", K), None);
        assert_eq!(get_kv("", K), None);
    }
}
