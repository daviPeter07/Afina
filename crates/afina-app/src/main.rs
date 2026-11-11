// No console window in release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![forbid(unsafe_code)]

//! Afina — a light, transparent and fully reversible Windows tuner for games.
//!
//! UI data flow (one direction only):
//! `pages/*` and `shell/*` draw from `&AppState` and emit [`action::Action`]s →
//! `controller` applies them (calling `afina-core`) → `AppState` is updated →
//! next frame redraws.

mod action;
mod activity;
mod app;
mod components;
mod controller;
mod dialogs;
mod icon;
mod jobs;
mod pages;
mod shell;
mod state;
mod theme;

use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Afina — Windows afinado para jogos")
            .with_inner_size([1060.0, 720.0])
            .with_min_inner_size([860.0, 560.0])
            .with_icon(icon::app_icon()),
        ..Default::default()
    };
    eframe::run_native("Afina", options, Box::new(|cc| Ok(Box::new(app::AfinaApp::new(cc)))))
}
