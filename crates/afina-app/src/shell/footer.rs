use eframe::egui::{Align, Layout, RichText, ScrollArea, Ui};

use crate::action::Action;
use crate::state::AppState;
use crate::theme::level_color;
use crate::theme::palette::{AMBER, TEXT};

/// Reboot banner, current activity / last message, and the expandable log.
pub fn footer(ui: &mut Ui, state: &AppState, out: &mut Vec<Action>) {
    if state.pending_reboot {
        ui.horizontal(|ui| {
            ui.label(RichText::new("⟳ Alguns ajustes só valem depois de reiniciar o PC.").color(AMBER));
            if ui.button("Reiniciar agora").clicked() {
                out.push(Action::AskReboot);
            }
        });
        ui.add_space(4.0);
    }

    ui.horizontal(|ui| {
        if let Some(tool) = state.busy {
            ui.spinner();
            ui.label(RichText::new(format!("{}...", tool.label())).color(TEXT));
        } else if let Some(last) = state.log.last() {
            ui.label(RichText::new(&last.text).color(level_color(last.level)));
        }
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            let label = if state.show_log { "Ocultar registro" } else { "Ver registro" };
            if ui.button(label).clicked() {
                out.push(Action::ToggleLog);
            }
        });
    });

    if state.show_log {
        ui.add_space(6.0);
        ScrollArea::vertical().max_height(150.0).stick_to_bottom(true).auto_shrink([false, true]).show(ui, |ui| {
            for line in state.log.iter() {
                let text = format!("{}  {}", line.time, line.text);
                ui.label(RichText::new(text).monospace().size(12.0).color(level_color(line.level)));
            }
        });
    }
}
