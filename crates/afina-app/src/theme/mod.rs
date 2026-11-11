//! Look & feel: palette, fonts and egui style. Call [`apply`] once at startup.

pub mod fonts;
pub mod palette;

use eframe::egui::{self, Color32, CornerRadius, FontFamily, FontId, Stroke, vec2};

pub use fonts::bold;
use palette::*;

use crate::activity::Level;

pub fn apply(ctx: &egui::Context) {
    fonts::install(ctx);
    ctx.all_styles_mut(|style| {
        style.text_styles.insert(egui::TextStyle::Body, FontId::new(14.5, FontFamily::Proportional));
        style.text_styles.insert(egui::TextStyle::Button, FontId::new(14.5, FontFamily::Proportional));
        style.spacing.item_spacing = vec2(8.0, 6.0);
        style.spacing.button_padding = vec2(12.0, 6.0);

        let v = &mut style.visuals;
        *v = egui::Visuals::dark();
        v.panel_fill = BG;
        v.window_fill = CARD;
        v.extreme_bg_color = Color32::from_rgb(10, 11, 16);
        v.selection.bg_fill = ACCENT;
        v.hyperlink_color = BLUE;
        v.widgets.noninteractive.fg_stroke = Stroke::new(1.0, TEXT);
        v.widgets.inactive.weak_bg_fill = WIDGET;
        v.widgets.inactive.bg_fill = WIDGET;
        v.widgets.hovered.weak_bg_fill = WIDGET_HOVER;
        v.widgets.active.weak_bg_fill = ACCENT;
        for w in [&mut v.widgets.inactive, &mut v.widgets.hovered, &mut v.widgets.active] {
            w.corner_radius = CornerRadius::same(8);
        }
    });
}

/// Text color for a log level.
pub fn level_color(level: Level) -> Color32 {
    match level {
        Level::Info => MUTED,
        Level::Ok => GREEN,
        Level::Err => RED,
    }
}
