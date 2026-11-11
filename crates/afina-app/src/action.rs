//! Everything the user can ask for. Views emit actions; only the controller
//! acts on them. Adding a feature usually starts by adding a variant here.

use afina_core::maintenance::Tool;
use afina_core::tweak::Tweak;

use crate::state::Page;

#[derive(Debug, Clone)]
pub enum Action {
    Navigate(Page),
    /// Turn a tweak on/off. Advanced tweaks go through a confirmation dialog first.
    SetTweak(&'static Tweak, bool),
    ApplyRecommended,
    SetRestoreFirst(bool),
    RunTool(Tool),
    /// Index into `AppState::startup`.
    SetStartup(usize, bool),
    RefreshStartup,
    AskUndoAll,
    AskReboot,
    /// Answer to the open confirmation dialog.
    Confirm(bool),
    RelaunchAsAdmin,
    ToggleLog,
    /// URI or folder opened with the shell (e.g. `ms-settings:gaming-gamemode`).
    Open(String),
    OpenDataFolder,
}
