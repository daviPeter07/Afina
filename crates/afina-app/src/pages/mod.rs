//! One file per sidebar page. Every page has the same signature:
//! `fn show(ui, &AppState, &mut Vec<Action>)` — read state, emit actions.

mod about;
mod home;
mod restore;
mod startup;
mod tools;
mod tweaks;

use afina_core::tweak::Category;
use eframe::egui::Ui;

use crate::action::Action;
use crate::state::{AppState, Page};

pub fn show(ui: &mut Ui, state: &AppState, out: &mut Vec<Action>) {
    match state.page {
        Page::Inicio => home::show(ui, state, out),
        Page::Jogos => tweaks::show(ui, state, out, Category::Jogos),
        Page::Sistema => tweaks::show(ui, state, out, Category::Sistema),
        Page::Visual => tweaks::show(ui, state, out, Category::Visual),
        Page::Inicializacao => startup::show(ui, state, out),
        Page::Ferramentas => tools::show(ui, state, out),
        Page::Restaurar => restore::show(ui, state, out),
        Page::Sobre => about::show(ui, state, out),
    }
}
