//! Restaurar: what Afina changed, "undo all", and where its files live.

use afina_core::{catalog, paths};
use eframe::egui::{RichText, Ui};

use crate::action::Action;
use crate::components::{card, danger_button, page_title, section_title};
use crate::state::AppState;
use crate::theme::palette::{MUTED, TEXT};

pub fn show(ui: &mut Ui, state: &AppState, out: &mut Vec<Action>) {
    page_title(ui, "Restaurar", "O Afina guarda os valores originais antes de mudar qualquer coisa.");

    let changed: Vec<&str> = state.store.changed_ids().collect();
    card(ui, |ui| {
        if changed.is_empty() {
            ui.label(RichText::new("Nenhum ajuste do Afina está ativo — o Windows está como antes.").color(MUTED));
            return;
        }
        section_title(ui, &format!("{} ajuste(s) feitos pelo Afina:", changed.len()));
        ui.add_space(4.0);
        for id in &changed {
            let name = catalog::find(id).map_or(*id, |t| t.title);
            ui.label(RichText::new(format!("•  {name}")).color(TEXT));
        }
        ui.add_space(8.0);
        if ui.add(danger_button("Desfazer todos os ajustes")).clicked() {
            out.push(Action::AskUndoAll);
        }
    });

    card(ui, |ui| {
        section_title(ui, "Arquivos do Afina");
        let backup = format!("Backup: {}", paths::backup_file().display());
        ui.label(RichText::new(backup).color(MUTED).monospace().size(12.0));
        ui.label(RichText::new("Registro de atividades: afina.log, na mesma pasta.").color(MUTED));
        if ui.button("Abrir pasta").clicked() {
            out.push(Action::OpenDataFolder);
        }
    });
}
