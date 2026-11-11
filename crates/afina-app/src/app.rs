//! `AfinaApp`: owns the state and background workers, lays out the window,
//! and runs the frame loop: poll → draw (collect actions) → dispatch.

use std::path::PathBuf;

use afina_core::backup::BackupStore;
use afina_core::{catalog, engine, paths, startup};
use afina_platform::{elevation, sysinfo};
use eframe::egui::{self, Margin, ScrollArea, Ui};

use crate::action::Action;
use crate::activity::ActivityLog;
use crate::jobs::{ClockProbe, JobRunner};
use crate::state::{AppState, ClockStatus, Page};
use crate::theme::palette::{BG, PANEL};
use crate::{dialogs, pages, shell, theme};

pub struct AfinaApp {
    pub(crate) state: AppState,
    pub(crate) jobs: JobRunner,
    pub(crate) clock_probe: ClockProbe,
    pub(crate) store_path: PathBuf,
}

impl AfinaApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        theme::apply(&cc.egui_ctx);
        let store_path = paths::backup_file();
        let page = std::env::var("AFINA_PAGINA").map(|p| Page::from_name(&p)).unwrap_or(Page::Inicio);
        let mut app = AfinaApp {
            state: AppState {
                page,
                sys: sysinfo::sys_info(),
                admin: elevation::is_admin(),
                store: BackupStore::load_from(&store_path),
                tweak_on: Default::default(),
                startup: Vec::new(),
                log: ActivityLog::new(Some(paths::log_file())),
                show_log: false,
                pending_reboot: false,
                confirm: None,
                restore_first: true,
                clock: ClockStatus::Checking,
                busy: None,
            },
            jobs: JobRunner::default(),
            clock_probe: ClockProbe::start(),
            store_path,
        };
        app.refresh_tweaks();
        app.refresh_startup();
        app.state.log.info("Afina iniciado. Nenhuma alteração é feita sem você clicar.");
        if !app.state.admin {
            app.state.log.info("Rodando sem administrador: ajustes marcados com \"Admin\" ficam bloqueados.");
        }
        app
    }

    /// Re-reads every tweak's real state from Windows.
    pub(crate) fn refresh_tweaks(&mut self) {
        self.state.tweak_on = catalog::all().map(|t| (t.id, engine::is_on(t, &self.state.store))).collect();
    }

    pub(crate) fn refresh_startup(&mut self) {
        self.state.startup = startup::list();
    }

    pub(crate) fn restart_clock_probe(&mut self) {
        self.state.clock = ClockStatus::Checking;
        self.clock_probe = ClockProbe::start();
    }
}

impl std::fmt::Debug for AfinaApp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AfinaApp").field("page", &self.state.page).finish_non_exhaustive()
    }
}

fn panel_frame(x: i8, y: i8) -> egui::Frame {
    egui::Frame::new().fill(PANEL).inner_margin(Margin::symmetric(x, y))
}

impl eframe::App for AfinaApp {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        self.poll_background(&ctx);

        let mut actions: Vec<Action> = Vec::new();
        let state = &self.state;
        egui::Panel::top("header").frame(panel_frame(18, 12)).show(ui, |ui| shell::header(ui, state, &mut actions));
        egui::Panel::bottom("footer").frame(panel_frame(18, 8)).show(ui, |ui| shell::footer(ui, state, &mut actions));
        egui::Panel::left("sidebar")
            .resizable(false)
            .exact_size(200.0)
            .frame(panel_frame(10, 14))
            .show(ui, |ui| shell::sidebar(ui, state, &mut actions));
        egui::CentralPanel::default().frame(egui::Frame::new().fill(BG).inner_margin(Margin::symmetric(22, 16))).show(
            ui,
            |ui| {
                ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| pages::show(ui, state, &mut actions));
            },
        );
        dialogs::show(&ctx, state, &mut actions);

        for action in actions {
            self.dispatch(action, &ctx);
        }
    }
}
