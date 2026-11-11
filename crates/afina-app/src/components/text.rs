use eframe::egui::{Label, RichText, Ui};

use crate::theme::bold;
use crate::theme::palette::{MUTED, TEXT};

/// Big title + muted subtitle at the top of a page.
pub fn page_title(ui: &mut Ui, title: &str, subtitle: &str) {
    ui.label(RichText::new(title).family(bold()).size(24.0).color(TEXT));
    ui.label(RichText::new(subtitle).color(MUTED));
    ui.add_space(12.0);
}

/// Bold heading inside a card.
pub fn section_title(ui: &mut Ui, text: &str) {
    ui.label(RichText::new(text).family(bold()).size(16.0).color(TEXT));
}

/// "Label    value" row with an aligned label column.
pub fn info_row(ui: &mut Ui, label: &str, value: &str) {
    ui.horizontal(|ui| {
        ui.add_sized([110.0, 18.0], Label::new(RichText::new(label).color(MUTED)));
        ui.label(RichText::new(value).color(TEXT));
    });
}
