//! Registry access with values handled as raw `(type, bytes)` pairs, so that any
//! original value can be backed up and restored byte-for-byte.

use std::borrow::Cow;
use std::io;

use serde::{Deserialize, Serialize};
use winreg::enums::{KEY_SET_VALUE, RegType};
use winreg::{HKCU, HKLM, RegKey, RegValue};

pub const REG_SZ: u32 = 1;
pub const REG_EXPAND_SZ: u32 = 2;
pub const REG_BINARY: u32 = 3;
pub const REG_DWORD: u32 = 4;

/// Root key. Only the two hives Afina needs are exposed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Hive {
    Hkcu,
    Hklm,
}

impl Hive {
    fn root(self) -> &'static RegKey {
        match self {
            Hive::Hkcu => HKCU,
            Hive::Hklm => HKLM,
        }
    }
}

/// A registry value exactly as stored: type code + raw bytes.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RawValue {
    pub vtype: u32,
    pub data: Vec<u8>,
}

impl RawValue {
    pub fn dword(v: u32) -> Self {
        RawValue { vtype: REG_DWORD, data: v.to_le_bytes().to_vec() }
    }

    pub fn string(s: &str) -> Self {
        let mut data: Vec<u8> = s.encode_utf16().flat_map(|c| c.to_le_bytes()).collect();
        data.extend_from_slice(&[0, 0]);
        RawValue { vtype: REG_SZ, data }
    }

    pub fn binary(bytes: &[u8]) -> Self {
        RawValue { vtype: REG_BINARY, data: bytes.to_vec() }
    }

    pub fn as_dword(&self) -> Option<u32> {
        match (self.vtype, self.data.as_slice()) {
            (REG_DWORD, [a, b, c, d, ..]) => Some(u32::from_le_bytes([*a, *b, *c, *d])),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<String> {
        if self.vtype != REG_SZ && self.vtype != REG_EXPAND_SZ {
            return None;
        }
        let units: Vec<u16> =
            self.data.as_chunks::<2>().0.iter().map(|c| u16::from_le_bytes(*c)).take_while(|&u| u != 0).collect();
        Some(String::from_utf16_lossy(&units))
    }
}

fn type_from_u32(t: u32) -> RegType {
    match t {
        1 => RegType::REG_SZ,
        2 => RegType::REG_EXPAND_SZ,
        3 => RegType::REG_BINARY,
        4 => RegType::REG_DWORD,
        5 => RegType::REG_DWORD_BIG_ENDIAN,
        6 => RegType::REG_LINK,
        7 => RegType::REG_MULTI_SZ,
        8 => RegType::REG_RESOURCE_LIST,
        9 => RegType::REG_FULL_RESOURCE_DESCRIPTOR,
        10 => RegType::REG_RESOURCE_REQUIREMENTS_LIST,
        11 => RegType::REG_QWORD,
        _ => RegType::REG_NONE,
    }
}

fn type_to_u32(t: &RegType) -> u32 {
    match t {
        RegType::REG_SZ => 1,
        RegType::REG_EXPAND_SZ => 2,
        RegType::REG_BINARY => 3,
        RegType::REG_DWORD => 4,
        RegType::REG_DWORD_BIG_ENDIAN => 5,
        RegType::REG_LINK => 6,
        RegType::REG_MULTI_SZ => 7,
        RegType::REG_RESOURCE_LIST => 8,
        RegType::REG_FULL_RESOURCE_DESCRIPTOR => 9,
        RegType::REG_RESOURCE_REQUIREMENTS_LIST => 10,
        RegType::REG_QWORD => 11,
        _ => 0,
    }
}

fn to_raw(v: &RegValue) -> RawValue {
    RawValue { vtype: type_to_u32(&v.vtype), data: v.bytes.to_vec() }
}

fn is_not_found(e: &io::Error) -> bool {
    e.kind() == io::ErrorKind::NotFound
}

/// Reads a value. `Ok(None)` when the key or the value does not exist.
pub fn read(hive: Hive, path: &str, name: &str) -> io::Result<Option<RawValue>> {
    let key = match hive.root().open_subkey(path) {
        Ok(k) => k,
        Err(e) if is_not_found(&e) => return Ok(None),
        Err(e) => return Err(e),
    };
    match key.get_raw_value(name) {
        Ok(v) => Ok(Some(to_raw(&v))),
        Err(e) if is_not_found(&e) => Ok(None),
        Err(e) => Err(e),
    }
}

/// Like [`read`], but any error counts as "missing". For display/state only.
pub fn read_opt(hive: Hive, path: &str, name: &str) -> Option<RawValue> {
    read(hive, path, name).ok().flatten()
}

pub fn read_string(hive: Hive, path: &str, name: &str) -> Option<String> {
    read_opt(hive, path, name).and_then(|v| v.as_string())
}

/// Writes a value, creating the key if needed.
pub fn write(hive: Hive, path: &str, name: &str, value: &RawValue) -> io::Result<()> {
    let (key, _) = hive.root().create_subkey(path)?;
    let rv = RegValue { bytes: Cow::Owned(value.data.clone()), vtype: type_from_u32(value.vtype) };
    key.set_raw_value(name, &rv)
}

/// Deletes a value. A missing key or value is not an error.
pub fn delete(hive: Hive, path: &str, name: &str) -> io::Result<()> {
    let key = match hive.root().open_subkey_with_flags(path, KEY_SET_VALUE) {
        Ok(k) => k,
        Err(e) if is_not_found(&e) => return Ok(()),
        Err(e) => return Err(e),
    };
    match key.delete_value(name) {
        Err(e) if is_not_found(&e) => Ok(()),
        other => other,
    }
}

/// Writes `Some(value)` or deletes on `None`.
pub fn put(hive: Hive, path: &str, name: &str, value: Option<&RawValue>) -> io::Result<()> {
    match value {
        Some(v) => write(hive, path, name, v),
        None => delete(hive, path, name),
    }
}

/// All values of a key (empty when the key is missing or unreadable).
pub fn list_values(hive: Hive, path: &str) -> Vec<(String, RawValue)> {
    let Ok(key) = hive.root().open_subkey(path) else {
        return Vec::new();
    };
    key.enum_values().filter_map(|r| r.ok()).map(|(name, v)| (name, to_raw(&v))).collect()
}

/// Names of the direct subkeys of a key.
pub fn list_subkeys(hive: Hive, path: &str) -> Vec<String> {
    let Ok(key) = hive.root().open_subkey(path) else {
        return Vec::new();
    };
    key.enum_keys().filter_map(|r| r.ok()).collect()
}

/// Root of the throw-away keys used by tests. Nothing else may ever be deleted recursively.
pub const TEST_ROOT: &str = r"Software\AfinaTest";

/// Deletes a whole key tree under `HKCU\Software\AfinaTest` (tests only).
///
/// # Panics
/// If `path` is outside [`TEST_ROOT`] — a guard against deleting real settings.
pub fn delete_test_tree(path: &str) {
    assert!(path.starts_with(TEST_ROOT), "delete_test_tree só pode apagar chaves de teste: {path}");
    let _ = HKCU.delete_subkey_all(path);
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn string_roundtrip() {
        let raw = RawValue::string("Olá, mundo");
        assert_eq!(raw.as_string().as_deref(), Some("Olá, mundo"));
        assert_eq!(raw.as_dword(), None);
    }

    #[test]
    fn dword_roundtrip() {
        assert_eq!(RawValue::dword(2).as_dword(), Some(2));
        assert_eq!(RawValue::binary(&[1, 2]).as_dword(), None);
    }

    #[test]
    #[should_panic(expected = "chaves de teste")]
    fn test_tree_guard_refuses_real_keys() {
        delete_test_tree(r"Software\Microsoft");
    }

    #[test]
    fn real_registry_read_write_delete() {
        let path = r"Software\AfinaTest\platform_registry";
        delete_test_tree(path);
        assert_eq!(read(Hive::Hkcu, path, "x").unwrap(), None);
        write(Hive::Hkcu, path, "x", &RawValue::dword(42)).unwrap();
        assert_eq!(read(Hive::Hkcu, path, "x").unwrap(), Some(RawValue::dword(42)));
        write(Hive::Hkcu, path, "s", &RawValue::string("abc")).unwrap();
        assert_eq!(read_string(Hive::Hkcu, path, "s").as_deref(), Some("abc"));
        assert_eq!(list_values(Hive::Hkcu, path).len(), 2);
        delete(Hive::Hkcu, path, "x").unwrap();
        delete(Hive::Hkcu, path, "x").unwrap(); // idempotent
        assert_eq!(read(Hive::Hkcu, path, "x").unwrap(), None);
        delete_test_tree(path);
    }
}
