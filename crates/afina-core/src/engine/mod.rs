//! The engine: reads a tweak's real state and applies / reverts it.
//!
//! Contract shared by every [`Kind`] implementation:
//! * state is always READ from Windows — never cached or assumed;
//! * before the first change, originals go into the [`BackupStore`] (first snapshot wins);
//! * `set(.., false)` restores the originals; if that still leaves the tweak on
//!   (Windows' default was already "on"), the explicit `off` values are written;
//! * the caller persists the store after each call.

mod dx_flag;
mod power_plan;
mod registry_ops;

pub use dx_flag::{get_kv, set_kv};
pub use power_plan::find_guid;

use afina_platform::notify;

use crate::Result;
use crate::backup::BackupStore;
use crate::tweak::{Kind, Post, Tweak};

/// Current state, read from Windows.
pub fn is_on(t: &Tweak, store: &BackupStore) -> bool {
    match t.kind {
        Kind::Registry(ops) => registry_ops::is_on(ops),
        Kind::DxGlobalFlag { key } => dx_flag::is_on(key),
        Kind::PowerPlan => power_plan::is_on(t.id, store),
    }
}

/// Turns a tweak on or off.
pub fn set(t: &Tweak, on: bool, store: &mut BackupStore) -> Result<()> {
    match (t.kind, on) {
        (Kind::Registry(ops), true) => registry_ops::turn_on(t.id, ops, store),
        (Kind::Registry(ops), false) => registry_ops::turn_off(t.id, ops, store),
        (Kind::DxGlobalFlag { key }, true) => dx_flag::turn_on(t.id, key, store),
        (Kind::DxGlobalFlag { key }, false) => dx_flag::turn_off(t.id, key, store),
        (Kind::PowerPlan, true) => power_plan::turn_on(t.id, store),
        (Kind::PowerPlan, false) => power_plan::turn_off(t.id, store),
    }?;
    notify_windows(t.post);
    Ok(())
}

/// Puts back exactly what was there before Afina (used by "Desfazer tudo").
/// Does nothing when the tweak has no backup.
pub fn restore_original(t: &Tweak, store: &mut BackupStore) -> Result<()> {
    let Some(backup) = store.tweaks.get(t.id).cloned() else {
        return Ok(());
    };
    match t.kind {
        Kind::Registry(_) => registry_ops::restore(&backup.registry)?,
        Kind::DxGlobalFlag { key } => dx_flag::restore(key, &backup)?,
        Kind::PowerPlan => power_plan::restore(&backup)?,
    }
    store.tweaks.remove(t.id);
    notify_windows(t.post);
    Ok(())
}

fn notify_windows(post: Post) {
    match post {
        Post::Mouse => notify::refresh_mouse_settings(),
        Post::Theme => notify::broadcast_theme_change(),
        Post::None => {}
    }
}
