//! Ferramentas: one-shot maintenance actions + shortcuts to Windows settings.

use afina_core::maintenance::Tool;
use eframe::egui::{Align, Layout, RichText, Ui};

use crate::action::Action;
use crate::components::{badge, card, page_title, primary_button, section_title};
use crate::state::AppState;
use crate::theme::palette::{BLUE, MUTED};

/// (button label, URI) of Windows settings pages relevant to gaming.
const SHORTCUTS: [(&str, &str); 6] = [
    ("Gráficos (GPU por jogo)", "ms-settings:display-advancedgraphics"),
    ("Modo de Jogo", "ms-settings:gaming-gamemode"),
    ("Tela / taxa de atualização", "ms-settings:display-advanced"),
    ("Energia", "ms-settings:powersleep"),
    ("Isolamento de núcleo", "windowsdefender://coreisolation"),
    ("Armazenamento", "ms-settings:storagesense"),
];

pub fn show(ui: &mut Ui, state: &AppState, out: &mut Vec<Action>) {
    page_title(ui, "Ferramentas", "Ações de uma vez só. Nada fica rodando em segundo plano.");
    for tool in Tool::ALL {
        tool_card(ui, state, out, tool);
    }

    ui.add_space(10.0);
    section_title(ui, "Atalhos para configurações do Windows");
    ui.add_space(4.0);
    ui.horizontal_wrapped(|ui| {
        for (label, uri) in SHORTCUTS {
            if ui.button(label).clicked() {
                out.push(Action::Open(uri.to_string()));
            }
        }
    });
}

fn tool_card(ui: &mut Ui, state: &AppState, out: &mut Vec<Action>, tool: Tool) {
    let usable = state.can_use(tool.needs_admin()) && state.busy.is_none();
    card(ui, |ui| {
        ui.horizontal(|ui| {
            let text_width = (ui.available_width() - 130.0).max(200.0);
            ui.vertical(|ui| {
                ui.set_width(text_width);
                ui.horizontal(|ui| {
                    section_title(ui, tool.label());
                    if tool.needs_admin() {
                        badge(ui, "Admin", BLUE);
                    }
                });
                ui.label(RichText::new(tool.description()).color(MUTED));
            });
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.add_enabled_ui(usable, |ui| {
                    if ui.add(primary_button("Executar")).clicked() {
                        out.push(Action::RunTool(tool));
                    }
                });
            });
        });
    });
}
