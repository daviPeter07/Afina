//! The only place colors are defined. Views must use these names, never raw RGB.

use eframe::egui::Color32;

pub const BG: Color32 = Color32::from_rgb(14, 16, 22);
pub const PANEL: Color32 = Color32::from_rgb(20, 23, 32);
pub const CARD: Color32 = Color32::from_rgb(28, 32, 45);
pub const CARD_BORDER: Color32 = Color32::from_rgb(34, 39, 55);
pub const WIDGET: Color32 = Color32::from_rgb(40, 45, 62);
pub const WIDGET_HOVER: Color32 = Color32::from_rgb(52, 58, 80);
pub const TOGGLE_OFF: Color32 = Color32::from_rgb(70, 76, 96);

pub const TEXT: Color32 = Color32::from_rgb(232, 234, 240);
pub const MUTED: Color32 = Color32::from_rgb(150, 156, 176);

pub const ACCENT: Color32 = Color32::from_rgb(124, 92, 255);
pub const GREEN: Color32 = Color32::from_rgb(61, 214, 140);
pub const AMBER: Color32 = Color32::from_rgb(245, 185, 66);
pub const RED: Color32 = Color32::from_rgb(255, 92, 108);
pub const BLUE: Color32 = Color32::from_rgb(78, 168, 255);
