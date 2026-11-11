use eframe::egui::{self, Color32, CornerRadius, Response, Sense, Stroke, StrokeKind, Ui, pos2, vec2};

use crate::theme::palette::{ACCENT, TOGGLE_OFF};

/// iOS-style switch showing `on`. Returns the response; the caller decides what
/// a click means (the value itself is not mutated — state lives elsewhere).
pub fn toggle(ui: &mut Ui, on: bool) -> Response {
    let (rect, response) = ui.allocate_exact_size(vec2(46.0, 26.0), Sense::click());
    if ui.is_rect_visible(rect) {
        let t = ui.ctx().animate_bool_responsive(response.id, on);
        let enabled = ui.is_enabled();
        let mut track = lerp_color(TOGGLE_OFF, ACCENT, t);
        if !enabled {
            track = track.gamma_multiply(0.4);
        }
        let stroke = if response.hovered() && enabled {
            Stroke::new(1.5, Color32::WHITE.gamma_multiply(0.5))
        } else {
            Stroke::NONE
        };
        let radius = (rect.height() / 2.0) as u8;
        ui.painter().rect(rect, CornerRadius::same(radius), track, stroke, StrokeKind::Inside);
        let knob_x = egui::lerp((rect.left() + 13.0)..=(rect.right() - 13.0), t);
        let knob = if enabled { Color32::WHITE } else { Color32::from_gray(150) };
        ui.painter().circle_filled(pos2(knob_x, rect.center().y), 10.0, knob);
    }
    response
}

fn lerp_color(a: Color32, b: Color32, t: f32) -> Color32 {
    let l = |x: u8, y: u8| (x as f32 + (y as f32 - x as f32) * t).round() as u8;
    Color32::from_rgb(l(a.r(), b.r()), l(a.g(), b.g()), l(a.b(), b.b()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lerp_color_endpoints() {
        assert_eq!(lerp_color(TOGGLE_OFF, ACCENT, 0.0), TOGGLE_OFF);
        assert_eq!(lerp_color(TOGGLE_OFF, ACCENT, 1.0), ACCENT);
    }
}
