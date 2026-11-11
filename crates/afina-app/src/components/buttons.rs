use eframe::egui::{Button, Color32, RichText, vec2};

use crate::theme::palette::{ACCENT, RED};

fn filled(text: &str, fill: Color32) -> Button<'static> {
    Button::new(RichText::new(text.to_string()).color(Color32::WHITE).size(15.0))
        .fill(fill)
        .corner_radius(8)
        .min_size(vec2(110.0, 34.0))
}

/// Main call to action of a card.
pub fn primary_button(text: &str) -> Button<'static> {
    filled(text, ACCENT)
}

/// Destructive / irreversible-looking actions (undo all, reboot, enable advanced).
pub fn danger_button(text: &str) -> Button<'static> {
    filled(text, RED.gamma_multiply(0.85))
}
