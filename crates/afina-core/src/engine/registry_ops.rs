//! [`Kind::Registry`](crate::tweak::Kind::Registry): a set of registry values.

use afina_platform::registry;

use crate::backup::{BackupStore, RegBackup, TweakBackup};
use crate::tweak::RegOp;
use crate::{Error, Result};

pub(super) fn is_on(ops: &[RegOp]) -> bool {
    ops.iter().all(|o| {
        let current = registry::read_opt(o.hive, o.path, o.name);
        o.on.matches(current.as_ref()) || (current.is_none() && o.missing_means_on)
    })
}

pub(super) fn turn_on(id: &str, ops: &[RegOp], store: &mut BackupStore) -> Result<()> {
    let mut originals = Vec::with_capacity(ops.len());
    for o in ops {
        let current = registry::read(o.hive, o.path, o.name).map_err(|e| Error::io(o.name, e))?;
        originals.push(RegBackup { hive: o.hive, path: o.path.into(), name: o.name.into(), original: current });
    }
    let first_time = !store.has(id);
    if first_time {
        store.tweaks.insert(id.into(), TweakBackup { registry: originals.clone(), power: None });
    }
    for (i, o) in ops.iter().enumerate() {
        if let Err(e) = registry::put(o.hive, o.path, o.name, o.on.to_raw().as_ref()) {
            // Roll back what was already written: never leave a half-applied tweak.
            let _ = restore(&originals[..i]);
            if first_time {
                store.tweaks.remove(id);
            }
            return Err(Error::io(o.name, e));
        }
    }
    Ok(())
}

pub(super) fn turn_off(id: &str, ops: &[RegOp], store: &mut BackupStore) -> Result<()> {
    if let Some(b) = store.tweaks.get(id).cloned() {
        restore(&b.registry)?;
        store.tweaks.remove(id);
    }
    // The original state may itself have been "on" (e.g. Game Mode is on by default):
    // the user asked for OFF, so write the explicit off values.
    if is_on(ops) {
        for o in ops {
            registry::put(o.hive, o.path, o.name, o.off.to_raw().as_ref()).map_err(|e| Error::io(o.name, e))?;
        }
    }
    Ok(())
}

pub(super) fn restore(items: &[RegBackup]) -> Result<()> {
    for b in items {
        registry::put(b.hive, &b.path, &b.name, b.original.as_ref()).map_err(|e| Error::io(b.name.clone(), e))?;
    }
    Ok(())
}
