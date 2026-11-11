//! [`Kind::PowerPlan`](crate::tweak::Kind::PowerPlan): a dedicated "Afina - Desempenho"
//! plan duplicated from Ultimate Performance (fallback: High Performance).
//!
//! The backup stores the previously active plan and the GUID we created, so
//! turning off re-activates the old plan and deletes ours.

use afina_platform::process;

use crate::backup::{BackupStore, PowerBackup, TweakBackup};
use crate::{Error, Result};

const ULTIMATE: &str = "e9a42b02-d5df-448d-aa00-03f14749eb61";
const HIGH_PERF: &str = "8c5e7fda-e8bf-4a96-9a85-cf2a2a4b30b9";
const BALANCED: &str = "381b4222-f694-41f0-9685-ff5bb260df2e";

/// First GUID (8-4-4-4-12 hex) in a text, lowercase. powercfg output is
/// localized, so we never parse words — only GUIDs.
pub fn find_guid(text: &str) -> Option<String> {
    const GROUPS: [usize; 5] = [8, 4, 4, 4, 12];
    const LEN: usize = 36;
    let b = text.as_bytes();
    (0..=b.len().checked_sub(LEN)?).find_map(|start| {
        let mut i = start;
        for (g, &len) in GROUPS.iter().enumerate() {
            if !b[i..i + len].iter().all(u8::is_ascii_hexdigit) {
                return None;
            }
            i += len;
            if g < GROUPS.len() - 1 {
                if b[i] != b'-' {
                    return None;
                }
                i += 1;
            }
        }
        Some(text[start..start + LEN].to_lowercase())
    })
}

fn powercfg(args: &[&str]) -> Result<String> {
    let what = format!("powercfg {}", args.join(" "));
    let out = process::run_hidden("powercfg", args).map_err(|e| Error::io(what.clone(), e))?;
    if out.success { Ok(out.text) } else { Err(Error::command(what, out.last_line())) }
}

fn active_scheme() -> Option<String> {
    powercfg(&["/getactivescheme"]).ok().and_then(|o| find_guid(&o))
}

fn backup_of(id: &str, store: &BackupStore) -> Option<PowerBackup> {
    store.tweaks.get(id).and_then(|b| b.power.clone())
}

pub(super) fn is_on(id: &str, store: &BackupStore) -> bool {
    backup_of(id, store).is_some_and(|p| active_scheme().as_deref() == Some(p.created_scheme.as_str()))
}

pub(super) fn turn_on(id: &str, store: &mut BackupStore) -> Result<()> {
    if let Some(p) = backup_of(id, store) {
        // Created earlier: just activate it again (it may have been deleted meanwhile).
        if powercfg(&["/setactive", &p.created_scheme]).is_ok() {
            return Ok(());
        }
        store.tweaks.remove(id);
    }
    let previous = active_scheme().unwrap_or_else(|| BALANCED.to_string());
    let created = [ULTIMATE, HIGH_PERF]
        .iter()
        .find_map(|base| powercfg(&["/duplicatescheme", base]).ok().and_then(|o| find_guid(&o)))
        .ok_or_else(|| {
            Error::Unsupported(
                "este PC não permite criar planos de desempenho (comum em notebooks com Modern Standby)".into(),
            )
        })?;
    let _ = powercfg(&["/changename", &created, "Afina - Desempenho", "Criado pelo Afina. Pode ser removido no app."]);
    if let Err(e) = powercfg(&["/setactive", &created]) {
        let _ = powercfg(&["/delete", &created]);
        return Err(e);
    }
    let power = PowerBackup { previous_scheme: previous, created_scheme: created };
    store.tweaks.insert(id.into(), TweakBackup { registry: Vec::new(), power: Some(power) });
    Ok(())
}

pub(super) fn turn_off(id: &str, store: &mut BackupStore) -> Result<()> {
    match backup_of(id, store) {
        Some(p) => restore_plan(&p)?,
        None => {
            powercfg(&["/setactive", BALANCED])?;
        }
    }
    store.tweaks.remove(id);
    Ok(())
}

pub(super) fn restore(backup: &TweakBackup) -> Result<()> {
    backup.power.as_ref().map_or(Ok(()), restore_plan)
}

fn restore_plan(p: &PowerBackup) -> Result<()> {
    if powercfg(&["/setactive", &p.previous_scheme]).is_err() {
        powercfg(&["/setactive", BALANCED])?;
    }
    let _ = powercfg(&["/delete", &p.created_scheme]);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_guid_in_localized_output() {
        let pt = "GUID do Esquema de Energia: 685B94F9-2b30-48ee-8f14-ab870bb48cdd  (Top do top)";
        assert_eq!(find_guid(pt).as_deref(), Some("685b94f9-2b30-48ee-8f14-ab870bb48cdd"));
        let en = "Power Scheme GUID: e9a42b02-d5df-448d-aa00-03f14749eb61  (Ultimate Performance)";
        assert_eq!(find_guid(en).as_deref(), Some(ULTIMATE));
    }

    #[test]
    fn rejects_non_guids() {
        assert_eq!(find_guid("nada aqui"), None);
        assert_eq!(find_guid("1234-5678"), None);
        assert_eq!(find_guid(""), None);
        assert_eq!(find_guid("e9a42b02-d5df-448d-aa00_03f14749eb61"), None);
    }
}
