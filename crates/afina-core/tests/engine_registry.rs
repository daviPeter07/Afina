//! Integration test of the engine against the REAL registry, using only
//! throw-away keys under `HKCU\Software\AfinaTest` (never real settings).
//! Each test uses its own key, so tests can run in parallel.
#![allow(clippy::unwrap_used)]

use afina_core::backup::BackupStore;
use afina_core::engine;
use afina_core::tweak::Val::{Delete, Dword, Sz};
use afina_core::tweak::{Category, Kind, Post, RegOp, Risk, Tweak};
use afina_platform::registry::{self, Hive, RawValue};

/// Declares a test tweak with two values ("Num" DWORD and "Txt" string) under `$path`.
macro_rules! test_tweak {
    ($name:ident, $id:literal, $path:literal) => {
        static $name: Tweak = Tweak {
            id: $id,
            category: Category::Sistema,
            title: "teste",
            summary: "teste",
            details: "",
            risk: Risk::Seguro,
            admin: false,
            reboot: false,
            recommended: false,
            kind: Kind::Registry(&[
                RegOp::new(Hive::Hkcu, $path, "Num", Dword(1), Dword(0)),
                RegOp::new(Hive::Hkcu, $path, "Txt", Sz("ligado"), Delete),
            ]),
            post: Post::None,
        };
    };
}

test_tweak!(APPLY_REVERT, "apply_revert", r"Software\AfinaTest\engine_apply_revert");
test_tweak!(ALREADY_ON, "already_on", r"Software\AfinaTest\engine_already_on");

fn path_of(t: &Tweak) -> &'static str {
    match t.kind {
        Kind::Registry(ops) => ops[0].path,
        _ => unreachable!(),
    }
}

fn read(t: &Tweak, name: &str) -> Option<RawValue> {
    registry::read(Hive::Hkcu, path_of(t), name).unwrap()
}

#[test]
fn apply_then_revert_restores_exact_originals() {
    let t = &APPLY_REVERT;
    registry::delete_test_tree(path_of(t));
    registry::write(Hive::Hkcu, path_of(t), "Num", &RawValue::dword(7)).unwrap();
    let mut store = BackupStore::default();

    assert!(!engine::is_on(t, &store));
    engine::set(t, true, &mut store).unwrap();
    assert!(engine::is_on(t, &store));

    // A second apply must keep the FIRST snapshot, not the current "on" values.
    engine::set(t, true, &mut store).unwrap();
    let snap = &store.tweaks[t.id].registry;
    assert_eq!(snap[0].original, Some(RawValue::dword(7)));
    assert_eq!(snap[1].original, None);

    engine::set(t, false, &mut store).unwrap();
    assert!(!engine::is_on(t, &store));
    assert!(!store.has(t.id));
    assert_eq!(read(t, "Num"), Some(RawValue::dword(7)));
    assert_eq!(read(t, "Txt"), None);

    registry::delete_test_tree(path_of(t));
}

#[test]
fn turning_off_something_already_on_writes_off_values() {
    let t = &ALREADY_ON;
    registry::delete_test_tree(path_of(t));
    registry::write(Hive::Hkcu, path_of(t), "Num", &RawValue::dword(1)).unwrap();
    registry::write(Hive::Hkcu, path_of(t), "Txt", &RawValue::string("ligado")).unwrap();
    let mut store = BackupStore::default();

    assert!(engine::is_on(t, &store), "já estava ligado, sem backup");
    engine::set(t, false, &mut store).unwrap();
    assert_eq!(read(t, "Num"), Some(RawValue::dword(0)));
    assert_eq!(read(t, "Txt"), None);

    // restore_original after an apply puts the snapshot (the "off" state) back.
    engine::set(t, true, &mut store).unwrap();
    engine::restore_original(t, &mut store).unwrap();
    assert_eq!(read(t, "Num"), Some(RawValue::dword(0)));
    assert!(!store.has(t.id));

    registry::delete_test_tree(path_of(t));
}
