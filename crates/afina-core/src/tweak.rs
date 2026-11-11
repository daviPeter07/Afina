//! Data model of a tweak. Tweaks are pure data (see [`crate::catalog`]);
//! all behaviour lives in [`crate::engine`].

use afina_platform::registry::{Hive, RawValue};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Category {
    Jogos,
    Sistema,
    Visual,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Risk {
    /// No downside for typical users.
    Seguro,
    /// Has a trade-off the description must explain.
    Moderado,
    /// Reduces security or may break something; asks for confirmation; never recommended.
    Avancado,
}

/// The value a tweak wants in the registry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Val {
    Dword(u32),
    Sz(&'static str),
    /// The value must not exist.
    Delete,
}

impl Val {
    pub fn to_raw(self) -> Option<RawValue> {
        match self {
            Val::Dword(v) => Some(RawValue::dword(v)),
            Val::Sz(s) => Some(RawValue::string(s)),
            Val::Delete => None,
        }
    }

    /// Does the current registry content (`None` = missing) match this value?
    pub fn matches(self, current: Option<&RawValue>) -> bool {
        match (self, current) {
            (Val::Delete, None) => true,
            (Val::Delete, Some(_)) | (_, None) => false,
            (Val::Dword(v), Some(raw)) => raw.as_dword() == Some(v),
            (Val::Sz(s), Some(raw)) => raw.as_string().as_deref() == Some(s),
        }
    }
}

/// One registry value controlled by a tweak.
#[derive(Clone, Copy, Debug)]
pub struct RegOp {
    pub hive: Hive,
    pub path: &'static str,
    pub name: &'static str,
    /// Value when the tweak is ON.
    pub on: Val,
    /// Value written to turn the tweak OFF when there is no backup to restore
    /// (or when the original state was already "on").
    pub off: Val,
    /// If the value is missing, Windows behaves as if it were `on`.
    pub missing_means_on: bool,
}

impl RegOp {
    pub const fn new(hive: Hive, path: &'static str, name: &'static str, on: Val, off: Val) -> Self {
        RegOp { hive, path, name, on, off, missing_means_on: false }
    }

    /// Marks that Windows' default (value absent) already equals `on`.
    pub const fn missing_means_on(mut self) -> Self {
        self.missing_means_on = true;
        self
    }
}

/// How a tweak is applied. Each variant has its own module in [`crate::engine`].
#[derive(Clone, Copy, Debug)]
pub enum Kind {
    /// A set of registry values.
    Registry(&'static [RegOp]),
    /// One `key=value;` flag inside `DirectXUserGlobalSettings` (shared with Windows' own UI).
    DxGlobalFlag { key: &'static str },
    /// A dedicated "Afina - Desempenho" power plan based on Ultimate Performance.
    PowerPlan,
}

/// What Windows must be told after a change, so it applies without signing out.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Post {
    None,
    Mouse,
    Theme,
}

#[derive(Debug)]
pub struct Tweak {
    /// Stable identifier. Stored in backup.json — never rename an existing id.
    pub id: &'static str,
    pub category: Category,
    pub title: &'static str,
    /// One sentence: what the user gains.
    pub summary: &'static str,
    /// Trade-offs, when to use, where the same option lives in Windows.
    pub details: &'static str,
    pub risk: Risk,
    pub admin: bool,
    pub reboot: bool,
    /// Part of "Aplicar recomendados": must be Seguro, need no admin and not change how Windows looks.
    pub recommended: bool,
    pub kind: Kind,
    pub post: Post,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn val_matching() {
        let two = RawValue::dword(2);
        assert!(Val::Dword(2).matches(Some(&two)));
        assert!(!Val::Dword(1).matches(Some(&two)));
        assert!(!Val::Dword(2).matches(None));
        assert!(Val::Delete.matches(None));
        assert!(!Val::Delete.matches(Some(&two)));
        assert!(Val::Sz("0").matches(Some(&RawValue::string("0"))));
        assert!(!Val::Sz("0").matches(Some(&two)));
    }

    #[test]
    fn regop_builder() {
        let op = RegOp::new(Hive::Hkcu, "a", "b", Val::Dword(1), Val::Dword(0));
        assert!(!op.missing_means_on);
        assert!(op.missing_means_on().missing_means_on);
    }
}
