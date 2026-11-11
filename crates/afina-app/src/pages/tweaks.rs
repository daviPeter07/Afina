//! Jogos / Sistema / Visual: one card with a switch per tweak of the category.

use afina_core::catalog;
use afina_core::tweak::{Category, Tweak};
use eframe::egui::{Align, CollapsingHeader, Layout, RichText, Ui};

use crate::action::Action;
use crate::components::{badge, card, page_title, risk_badge, section_title, toggle};
use crate::state::AppState;
use crate::theme::palette::{ACCENT, BLUE, GREEN, MUTED, TEXT};

/// Room kept on the right of each card for the switch.
const SWITCH_COLUMN: f32 = 70.0;

pub fn show(ui: &mut Ui, state: &AppState, out: &mut Vec<Action>, category: Category) {
    let (title, subtitle) = match category {
        Category::Jogos => ("Jogos", "Ajustes que afetam diretamente o desempenho e a resposta nos jogos."),
        Category::Sistema => ("Sistema", "Ajustes do Windows que liberam recursos ou resolvem problemas específicos."),
        Category::Visual => ("Visual", "Efeitos visuais da área de trabalho. Úteis em PCs com vídeo integrado."),
    };
    page_title(ui, title, subtitle);
    for t in catalog::by_category(category) {
        tweak_card(ui, state, out, t);
    }
}

fn tweak_card(ui: &mut Ui, state: &AppState, out: &mut Vec<Action>, t: &'static Tweak) {
    let on = state.is_on(t.id);
    let usable = state.can_use(t.admin);
    card(ui, |ui| {
        ui.horizontal_top(|ui| {
            let text_width = (ui.available_width() - SWITCH_COLUMN).max(200.0);
            ui.vertical(|ui| {
                ui.set_width(text_width);
                section_title(ui, t.title);
                ui.horizontal_wrapped(|ui| {
                    risk_badge(ui, t.risk);
                    if t.admin {
                        badge(ui, "Admin", BLUE);
                    }
                    if t.reboot {
                        badge(ui, "Reiniciar", ACCENT);
                    }
                    if t.recommended {
                        badge(ui, "Recomendado", GREEN);
                    }
                });
                ui.add_space(2.0);
                ui.label(RichText::new(t.summary).color(TEXT));
                if !t.details.is_empty() {
                    CollapsingHeader::new(RichText::new("Detalhes").color(MUTED).size(13.0)).id_salt(t.id).show(
                        ui,
                        |ui| {
                            ui.label(RichText::new(t.details).color(MUTED));
                        },
                    );
                }
            });
            ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                ui.add_space(4.0);
                let r = ui.add_enabled(usable, |ui: &mut Ui| toggle(ui, on));
                if r.on_disabled_hover_text("Requer administrador — use o botão no topo.").clicked() {
                    out.push(Action::SetTweak(t, !on));
                }
            });
        });
    });
}
