//! Início: system summary, clock health, one-click recommended tweaks.

use afina_core::catalog;
use afina_core::maintenance::Tool;
use eframe::egui::{Align, Layout, RichText, Ui};

use crate::action::Action;
use crate::components::{badge, card, info_row, page_title, primary_button, section_title};
use crate::state::{AppState, ClockStatus};
use crate::theme::palette::{GREEN, MUTED, RED};

pub fn show(ui: &mut Ui, state: &AppState, out: &mut Vec<Action>) {
    page_title(
        ui,
        "Deixe o Windows pronto para jogar",
        "Tudo aqui é opcional, explicado e pode ser desfeito na aba Restaurar.",
    );
    computer_card(ui, state);
    clock_card(ui, state, out);
    recommended_card(ui, state, out);
    next_steps_card(ui);
}

fn computer_card(ui: &mut Ui, state: &AppState) {
    card(ui, |ui| {
        section_title(ui, "Seu computador");
        ui.add_space(6.0);
        info_row(ui, "Sistema", &state.sys.os);
        info_row(ui, "Processador", &state.sys.cpu);
        let gpu = if state.sys.gpus.is_empty() { "não identificada".to_string() } else { state.sys.gpus.join(" + ") };
        info_row(ui, "Vídeo", &gpu);
        info_row(ui, "Memória", &format!("{:.0} GB", state.sys.ram_gb.round()));
    });
}

/// Wrong clocks make online games reject logins (e.g. VALORANT VAL 7).
fn clock_card(ui: &mut Ui, state: &AppState, out: &mut Vec<Action>) {
    card(ui, |ui| {
        ui.horizontal(|ui| {
            section_title(ui, "Relógio do Windows");
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| match &state.clock {
                ClockStatus::Checking => {
                    ui.spinner();
                }
                c if c.is_wrong() => badge(ui, "Errado", RED),
                ClockStatus::Measured(_) => badge(ui, "Certo", GREEN),
                ClockStatus::Unknown(_) => badge(ui, "Sem verificação", MUTED),
            });
        });
        let text = match &state.clock {
            ClockStatus::Checking => "Comparando com o horário da internet...".to_string(),
            ClockStatus::Measured(d) if state.clock.is_wrong() => format!(
                "O relógio está {} fora da hora certa. Isso faz jogos online recusarem o login (ex.: VAL 7 no VALORANT).",
                human_duration(*d)
            ),
            ClockStatus::Measured(d) => format!("Diferença de apenas {} s em relação à hora certa.", decimal_br(*d, 1)),
            ClockStatus::Unknown(e) => format!("Não foi possível verificar ({e})."),
        };
        ui.label(RichText::new(text).color(MUTED));
        if state.clock.is_wrong() {
            ui.add_space(6.0);
            ui.add_enabled_ui(state.admin && state.busy.is_none(), |ui| {
                let r =
                    ui.add(primary_button("Acertar o relógio agora")).on_disabled_hover_text("Requer administrador");
                if r.clicked() {
                    out.push(Action::RunTool(Tool::SyncClock));
                }
            });
        }
    });
}

fn recommended_card(ui: &mut Ui, state: &AppState, out: &mut Vec<Action>) {
    card(ui, |ui| {
        section_title(ui, "⚡ Ajustes recomendados");
        ui.label(
            RichText::new(
                "Seguros para qualquer PC, não precisam de administrador e não mudam a aparência do Windows:",
            )
            .color(MUTED),
        );
        ui.add_space(4.0);
        for t in catalog::recommended() {
            let (icon, color) = if state.is_on(t.id) { ("✔", GREEN) } else { ("○", MUTED) };
            ui.label(RichText::new(format!("{icon}  {}", t.title)).color(color));
        }
        ui.add_space(8.0);
        ui.add_enabled_ui(state.admin, |ui| {
            let mut restore_first = state.restore_first;
            let r = ui.checkbox(&mut restore_first, "Criar ponto de restauração do Windows antes");
            if r.on_disabled_hover_text("Requer administrador").changed() {
                out.push(Action::SetRestoreFirst(restore_first));
            }
        });
        ui.add_space(4.0);
        ui.add_enabled_ui(state.busy.is_none(), |ui| {
            if ui.add(primary_button("Aplicar recomendados")).clicked() {
                out.push(Action::ApplyRecommended);
            }
        });
    });
}

fn next_steps_card(ui: &mut Ui) {
    card(ui, |ui| {
        section_title(ui, "Próximos passos");
        for tip in [
            "Veja Jogos, Sistema e Visual para ajustes opcionais — cada um explica prós e contras.",
            "Em Inicialização, desligue programas que abrem sozinhos e roubam memória.",
            "Notebook? Jogue na tomada e use o modo de desempenho do fabricante.",
        ] {
            ui.label(RichText::new(format!("•  {tip}")).color(MUTED));
        }
    });
}

/// Number with Brazilian decimal comma.
fn decimal_br(v: f64, places: usize) -> String {
    format!("{v:.places$}").replace('.', ",")
}

fn human_duration(secs: f64) -> String {
    if secs >= 3600.0 {
        format!("{} h", decimal_br(secs / 3600.0, 1))
    } else if secs >= 60.0 {
        format!("{secs_min:.0} min", secs_min = secs / 60.0)
    } else {
        format!("{secs:.0} s")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brazilian_number_formatting() {
        assert_eq!(decimal_br(0.04, 1), "0,0");
        assert_eq!(human_duration(14_400.0), "4,0 h");
        assert_eq!(human_duration(125.0), "2 min");
        assert_eq!(human_duration(42.0), "42 s");
    }
}
