//! Everything the views need to draw. Views get `&AppState` (read-only);
//! only the controller mutates it.

use std::collections::HashMap;

use afina_core::backup::BackupStore;
use afina_core::maintenance::Tool;
use afina_core::startup::StartupItem;
use afina_core::tweak::Tweak;
use afina_platform::sysinfo::SysInfo;

use crate::activity::ActivityLog;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Page {
    Inicio,
    Jogos,
    Sistema,
    Visual,
    Inicializacao,
    Ferramentas,
    Restaurar,
    Sobre,
}

impl Page {
    pub const ALL: [Page; 8] = [
        Page::Inicio,
        Page::Jogos,
        Page::Sistema,
        Page::Visual,
        Page::Inicializacao,
        Page::Ferramentas,
        Page::Restaurar,
        Page::Sobre,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Page::Inicio => "🏠  Início",
            Page::Jogos => "🎮  Jogos",
            Page::Sistema => "⚙  Sistema",
            Page::Visual => "🎨  Visual",
            Page::Inicializacao => "🚀  Inicialização",
            Page::Ferramentas => "🔧  Ferramentas",
            Page::Restaurar => "↩  Restaurar",
            Page::Sobre => "ℹ  Sobre",
        }
    }

    /// Parses the `AFINA_PAGINA` value (unknown names fall back to Início).
    pub fn from_name(name: &str) -> Page {
        match name.trim().to_lowercase().as_str() {
            "jogos" => Page::Jogos,
            "sistema" => Page::Sistema,
            "visual" => Page::Visual,
            "inicializacao" => Page::Inicializacao,
            "ferramentas" => Page::Ferramentas,
            "restaurar" => Page::Restaurar,
            "sobre" => Page::Sobre,
            _ => Page::Inicio,
        }
    }
}

/// Dialogs that need an explicit yes/no.
#[derive(Debug, Clone)]
pub enum Confirm {
    EnableAdvanced(&'static Tweak),
    UndoAll,
    Reboot,
}

/// Result of comparing the PC clock with internet time.
#[derive(Debug, Clone)]
pub enum ClockStatus {
    Checking,
    /// Absolute difference in seconds.
    Measured(f64),
    Unknown(String),
}

impl ClockStatus {
    /// More than this and online games start rejecting logins.
    pub const TOLERANCE_SECS: f64 = 30.0;

    pub fn is_wrong(&self) -> bool {
        matches!(self, ClockStatus::Measured(d) if *d > Self::TOLERANCE_SECS)
    }
}

#[derive(Debug)]
pub struct AppState {
    pub page: Page,
    pub sys: SysInfo,
    pub admin: bool,
    pub store: BackupStore,
    /// Cached `engine::is_on` per tweak id; refreshed after every change.
    pub tweak_on: HashMap<&'static str, bool>,
    pub startup: Vec<StartupItem>,
    pub log: ActivityLog,
    pub show_log: bool,
    pub pending_reboot: bool,
    pub confirm: Option<Confirm>,
    pub restore_first: bool,
    pub clock: ClockStatus,
    /// Tool currently running in the background, if any.
    pub busy: Option<Tool>,
}

impl AppState {
    pub fn is_on(&self, id: &str) -> bool {
        self.tweak_on.get(id).copied().unwrap_or(false)
    }

    pub fn can_use(&self, needs_admin: bool) -> bool {
        self.admin || !needs_admin
    }

    pub fn active_count(&self) -> usize {
        self.tweak_on.values().filter(|on| **on).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_from_name() {
        assert_eq!(Page::from_name("jogos"), Page::Jogos);
        assert_eq!(Page::from_name(" Inicializacao "), Page::Inicializacao);
        assert_eq!(Page::from_name("???"), Page::Inicio);
    }

    #[test]
    fn clock_tolerance() {
        assert!(!ClockStatus::Measured(0.01).is_wrong());
        assert!(ClockStatus::Measured(14_400.0).is_wrong());
        assert!(!ClockStatus::Checking.is_wrong());
    }
}
