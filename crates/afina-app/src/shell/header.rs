use eframe::egui::{Align, Button, Color32, Layout, RichText, Ui};

use crate::action::Action;
use crate::components::badge;
use crate::state::AppState;
use crate::theme::bold;
use crate::theme::palette::{ACCENT, GREEN, MUTED};

pub fn header(ui: &mut Ui, state: &AppState, out: &mut Vec<Action>) {
    ui.horizontal(|ui| {
        ui.label(RichText::new("Afina").family(bold()).size(26.0).color(ACCENT));
        ui.add_space(6.0);
        ui.vertical(|ui| {
            ui.add_space(4.0);
            ui.label(RichText::new("Windows afinado para jogos — leve, transparente e reversível").color(MUTED));
        });
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if state.admin {
                badge(ui, "🛡 Administrador", GREEN);
                return;
            }
            let button = Button::new(RichText::new("🛡 Abrir como administrador").color(Color32::WHITE))
                .fill(ACCENT)
                .corner_radius(8);
            let hint = "Libera os ajustes marcados com \"Admin\". O Windows vai pedir confirmação.";
            if ui.add(button).on_hover_text(hint).clicked() {
                out.push(Action::RelaunchAsAdmin);
            }
        });
    });
}
