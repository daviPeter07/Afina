//! The ONLY place that mutates `AppState` or calls `afina-core` operations
//! with side effects. Views emit [`Action`]s; [`AfinaApp::dispatch`] handles them.

use afina_core::maintenance::Tool;
use afina_core::tweak::{Risk, Tweak};
use afina_core::{catalog, engine, paths, startup};
use afina_platform::{elevation, process};
use eframe::egui;

use crate::action::Action;
use crate::app::AfinaApp;
use crate::jobs::{After, Finished};
use crate::state::{ClockStatus, Confirm};

impl AfinaApp {
    pub(crate) fn dispatch(&mut self, action: Action, ctx: &egui::Context) {
        match action {
            Action::Navigate(page) => {
                self.state.page = page;
                if page == crate::state::Page::Inicializacao {
                    self.refresh_startup();
                }
            }
            Action::SetTweak(t, on) => {
                if on && t.risk == Risk::Avancado {
                    self.state.confirm = Some(Confirm::EnableAdvanced(t));
                } else {
                    self.set_tweak(t, on);
                }
            }
            Action::ApplyRecommended => {
                if self.state.admin && self.state.restore_first {
                    self.start_tool(Tool::RestorePoint, After::ApplyRecommended);
                } else {
                    self.apply_recommended();
                }
            }
            Action::SetRestoreFirst(v) => self.state.restore_first = v,
            Action::RunTool(tool) => self.start_tool(tool, After::Nothing),
            Action::SetStartup(index, enabled) => self.set_startup(index, enabled),
            Action::RefreshStartup => self.refresh_startup(),
            Action::AskUndoAll => self.state.confirm = Some(Confirm::UndoAll),
            Action::AskReboot => self.state.confirm = Some(Confirm::Reboot),
            Action::Confirm(yes) => self.answer_confirm(yes),
            Action::RelaunchAsAdmin => match elevation::relaunch_as_admin() {
                Ok(()) => ctx.send_viewport_cmd(egui::ViewportCommand::Close),
                Err(_) => self.state.log.err("Permissão de administrador não concedida."),
            },
            Action::ToggleLog => self.state.show_log = !self.state.show_log,
            Action::Open(target) => self.open(&target),
            Action::OpenDataFolder => {
                let dir = paths::data_dir();
                let _ = std::fs::create_dir_all(&dir);
                self.open(&dir.display().to_string());
            }
        }
    }

    /// Called every frame: collects results of background work.
    pub(crate) fn poll_background(&mut self, ctx: &egui::Context) {
        if let Some(finished) = self.jobs.poll() {
            self.on_job_finished(finished);
        }
        if let Some(result) = self.clock_probe.poll() {
            self.state.clock = match result {
                Ok(offset) => ClockStatus::Measured(offset.abs()),
                Err(e) => ClockStatus::Unknown(e.to_string()),
            };
        }
        self.state.busy = self.jobs.busy();
        if self.state.busy.is_some() || self.clock_probe.is_running() {
            ctx.request_repaint_after(std::time::Duration::from_millis(150));
        }
    }

    // ── tweaks ──

    fn set_tweak(&mut self, t: &'static Tweak, on: bool) {
        match engine::set(t, on, &mut self.state.store) {
            Ok(()) => {
                self.state.log.ok(format!("{}: {}.", t.title, if on { "ativado" } else { "desativado" }));
                self.state.pending_reboot |= t.reboot;
            }
            Err(e) => self.state.log.err(format!("{}: {e}", t.title)),
        }
        self.save_store();
        self.state.tweak_on.insert(t.id, engine::is_on(t, &self.state.store));
    }

    fn apply_recommended(&mut self) {
        let pending: Vec<&'static Tweak> =
            catalog::recommended().filter(|t| !self.state.is_on(t.id) && self.state.can_use(t.admin)).collect();
        if pending.is_empty() {
            self.state.log.ok("Os ajustes recomendados já estavam ativos.");
            return;
        }
        for t in &pending {
            self.set_tweak(t, true);
        }
        self.state.log.ok(format!("{} ajuste(s) recomendado(s) aplicados.", pending.len()));
    }

    fn undo_all(&mut self) {
        let ids: Vec<String> = self.state.store.changed_ids().map(String::from).collect();
        for id in ids {
            let Some(t) = catalog::find(&id) else {
                self.state.store.tweaks.remove(&id); // tweak removed from the catalog: drop its stale backup
                continue;
            };
            if !self.state.can_use(t.admin) {
                self.state.log.err(format!("{}: precisa de administrador para desfazer.", t.title));
                continue;
            }
            match engine::restore_original(t, &mut self.state.store) {
                Ok(()) => {
                    self.state.log.ok(format!("{}: valor original restaurado.", t.title));
                    self.state.pending_reboot |= t.reboot;
                }
                Err(e) => self.state.log.err(format!("{}: {e}", t.title)),
            }
        }
        self.save_store();
        self.refresh_tweaks();
    }

    // ── confirmations ──

    fn answer_confirm(&mut self, yes: bool) {
        let Some(confirm) = self.state.confirm.take() else { return };
        if !yes {
            return;
        }
        match confirm {
            Confirm::EnableAdvanced(t) => self.set_tweak(t, true),
            Confirm::UndoAll => self.undo_all(),
            Confirm::Reboot => {
                if let Err(e) = process::reboot_now() {
                    self.state.log.err(format!("Não foi possível reiniciar: {e}"));
                }
            }
        }
    }

    // ── background tools ──

    fn start_tool(&mut self, tool: Tool, after: After) {
        if self.jobs.start(tool, after) {
            self.state.busy = Some(tool);
            self.state.log.info(format!("{}...", tool.label()));
        }
    }

    fn on_job_finished(&mut self, f: Finished) {
        let ok = f.result.is_ok();
        match f.result {
            Ok(msg) => self.state.log.ok(msg),
            Err(e) => self.state.log.err(format!("{}: {e}", f.tool.label())),
        }
        if f.tool == Tool::SyncClock {
            self.restart_clock_probe();
        }
        if f.after == After::ApplyRecommended {
            if !ok {
                self.state.log.info("Continuando sem ponto de restauração (tudo ainda pode ser desfeito pelo Afina).");
            }
            self.apply_recommended();
        }
    }

    // ── startup apps ──

    fn set_startup(&mut self, index: usize, enabled: bool) {
        let Some(item) = self.state.startup.get(index).cloned() else { return };
        match startup::set_enabled(&item, enabled) {
            Ok(()) => {
                let what = if enabled { "vai abrir com o Windows" } else { "não abre mais com o Windows" };
                self.state.log.ok(format!("{} {what}.", item.name));
            }
            Err(e) => self.state.log.err(e.to_string()),
        }
        self.refresh_startup();
    }

    // ── helpers ──

    fn open(&mut self, target: &str) {
        if let Err(e) = process::open(target) {
            self.state.log.err(format!("Não foi possível abrir {target}: {e}"));
        }
    }

    fn save_store(&mut self) {
        if let Err(e) = self.state.store.save_to(&self.store_path) {
            self.state.log.err(format!("Não foi possível salvar o backup: {e}"));
        }
    }
}
