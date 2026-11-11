use afina_core::catalog;
use eframe::egui::{Align, Button, Color32, Layout, RichText, Ui, vec2};

use crate::action::Action;
use crate::state::{AppState, Page};
use crate::theme::palette::{ACCENT, MUTED};

pub fn sidebar(ui: &mut Ui, state: &AppState, out: &mut Vec<Action>) {
    for page in Page::ALL {
        let selected = state.page == page;
        let text = RichText::new(page.label()).size(15.0).color(if selected { Color32::WHITE } else { MUTED });
        let button = Button::new(text)
            .fill(if selected { ACCENT } else { Color32::TRANSPARENT })
            .corner_radius(8)
            .min_size(vec2(ui.available_width(), 38.0));
        if ui.add(button).clicked() && !selected {
            out.push(Action::Navigate(page));
        }
        ui.add_space(2.0);
    }
    ui.with_layout(Layout::bottom_up(Align::Min), |ui| {
        let summary = format!("{} de {} ajustes ativos", state.active_count(), catalog::count());
        ui.label(RichText::new(summary).color(MUTED).size(12.0));
    });
}
