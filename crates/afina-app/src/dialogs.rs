//! The yes/no confirmation dialog for `AppState::confirm`.

use eframe::egui::{self, Id, Margin, RichText};

use crate::action::Action;
use crate::components::danger_button;
use crate::state::{AppState, Confirm};
use crate::theme::bold;
use crate::theme::palette::{CARD, MUTED};

pub fn show(ctx: &egui::Context, state: &AppState, out: &mut Vec<Action>) {
    let Some(confirm) = &state.confirm else { return };
    let (heading, body, yes) = texts(confirm);
    let modal = egui::Modal::new(Id::unique("afina_confirm"))
        .frame(egui::Frame::new().fill(CARD).corner_radius(12).inner_margin(Margin::same(20)))
        .show(ctx, |ui| {
            ui.set_max_width(440.0);
            ui.label(RichText::new(heading).family(bold()).size(18.0));
            ui.add_space(6.0);
            ui.label(RichText::new(body).color(MUTED));
            ui.add_space(14.0);
            ui.horizontal(|ui| {
                if ui.add(danger_button(yes)).clicked() {
                    out.push(Action::Confirm(true));
                }
                if ui.button("Cancelar").clicked() {
                    out.push(Action::Confirm(false));
                }
            });
        });
    // Esc or click outside = cancel.
    if modal.should_close() && !out.iter().any(|a| matches!(a, Action::Confirm(_))) {
        out.push(Action::Confirm(false));
    }
}

fn texts(confirm: &Confirm) -> (String, String, &'static str) {
    match confirm {
        Confirm::EnableAdvanced(t) => (t.title.to_string(), t.details.to_string(), "Entendi, ativar"),
        Confirm::UndoAll => (
            "Desfazer todos os ajustes?".into(),
            "Todos os valores alterados pelo Afina voltam a ser como eram antes.".into(),
            "Desfazer tudo",
        ),
        Confirm::Reboot => (
            "Reiniciar o computador agora?".into(),
            "Salve seus trabalhos abertos antes de continuar.".into(),
            "Reiniciar",
        ),
    }
}
