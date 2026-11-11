use afina_core::tweak::Risk;
use eframe::egui::{self, Color32, Margin, RichText, Ui};

use crate::theme::palette::{AMBER, GREEN, RED};

/// Small colored pill with a label.
pub fn badge(ui: &mut Ui, text: &str, color: Color32) {
    egui::Frame::new().fill(color.gamma_multiply(0.18)).corner_radius(6).inner_margin(Margin::symmetric(7, 2)).show(
        ui,
        |ui| {
            ui.label(RichText::new(text).size(11.5).color(color));
        },
    );
}

pub fn risk_badge(ui: &mut Ui, risk: Risk) {
    match risk {
        Risk::Seguro => badge(ui, "Seguro", GREEN),
        Risk::Moderado => badge(ui, "Moderado", AMBER),
        Risk::Avancado => badge(ui, "Avançado", RED),
    }
}
