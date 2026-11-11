use eframe::egui::{self, Margin, Stroke, Ui};

use crate::theme::palette::{CARD, CARD_BORDER};

/// Full-width rounded panel; the basic building block of every page.
pub fn card(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
    egui::Frame::new()
        .fill(CARD)
        .corner_radius(12)
        .inner_margin(Margin::same(16))
        .stroke(Stroke::new(1.0, CARD_BORDER))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            add_contents(ui);
        });
    ui.add_space(10.0);
}
