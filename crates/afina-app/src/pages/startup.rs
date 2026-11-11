//! Inicialização: startup apps with a switch each.

use eframe::egui::{Align, Layout, RichText, Ui};

use crate::action::Action;
use crate::components::{badge, card, page_title, section_title, toggle};
use crate::state::AppState;
use crate::theme::palette::{BLUE, MUTED};

const MAX_COMMAND_CHARS: usize = 110;

pub fn show(ui: &mut Ui, state: &AppState, out: &mut Vec<Action>) {
    page_title(
        ui,
        "Programas que abrem com o Windows",
        "Desligar aqui só impede o programa de abrir sozinho — nada é desinstalado. Mesmo mecanismo do Gerenciador de Tarefas.",
    );
    ui.horizontal(|ui| {
        if ui.button("⟳ Atualizar lista").clicked() {
            out.push(Action::RefreshStartup);
        }
        let enabled = state.startup.iter().filter(|i| i.enabled).count();
        ui.label(RichText::new(format!("{enabled} de {} abrindo com o Windows", state.startup.len())).color(MUTED));
    });
    ui.add_space(8.0);
    if state.startup.is_empty() {
        ui.label(RichText::new("Nenhum programa encontrado.").color(MUTED));
    }

    for (index, item) in state.startup.iter().enumerate() {
        let usable = state.can_use(item.source.needs_admin());
        card(ui, |ui| {
            ui.horizontal(|ui| {
                let text_width = (ui.available_width() - 70.0).max(200.0);
                ui.vertical(|ui| {
                    ui.set_width(text_width);
                    ui.horizontal(|ui| {
                        section_title(ui, &item.name);
                        badge(ui, item.source.scope_label(), if item.source.needs_admin() { BLUE } else { MUTED });
                    });
                    ui.label(RichText::new(truncate(&item.command, MAX_COMMAND_CHARS)).color(MUTED).size(12.0));
                });
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    let r = ui.add_enabled(usable, |ui: &mut Ui| toggle(ui, item.enabled));
                    if r.on_disabled_hover_text("Requer administrador").clicked() {
                        out.push(Action::SetStartup(index, !item.enabled));
                    }
                });
            });
        });
    }
}

fn truncate(text: &str, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        return text.to_string();
    }
    text.chars().take(max_chars - 3).collect::<String>() + "..."
}

#[cfg(test)]
mod tests {
    #[test]
    fn truncate_counts_chars_not_bytes() {
        assert_eq!(super::truncate("ação", 10), "ação");
        assert_eq!(super::truncate("çççççç", 5), "çç...");
    }
}
