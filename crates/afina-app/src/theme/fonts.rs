//! Fonts: Segoe UI (ships with Windows) for text, Segoe UI Symbol right after it
//! for icons/symbols (✔ ⚡ ⟳ 🏠 ...), egui's bundled fonts as last resort.
//! A missing system font is simply skipped — never an error.

use std::sync::Arc;

use eframe::egui::{self, FontData, FontDefinitions, FontFamily};

/// Font family for titles (Segoe UI Semibold).
pub fn bold() -> FontFamily {
    FontFamily::Name("bold".into())
}

fn load(file: &str) -> Option<Arc<FontData>> {
    let windir = std::env::var("WINDIR").unwrap_or_else(|_| r"C:\Windows".into());
    std::fs::read(format!(r"{windir}\Fonts\{file}")).ok().map(|bytes| Arc::new(FontData::from_owned(bytes)))
}

pub fn install(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();
    // Prepended in reverse priority order, so the final order is: segoe, symbol, egui defaults.
    let mut proportional_front: Vec<String> = Vec::new();
    for (name, file) in [("segoe", "segoeui.ttf"), ("segoe_symbol", "seguisym.ttf")] {
        if let Some(data) = load(file) {
            fonts.font_data.insert(name.into(), data);
            proportional_front.push(name.into());
        }
    }
    let proportional = fonts.families.entry(FontFamily::Proportional).or_default();
    for (i, name) in proportional_front.iter().enumerate() {
        proportional.insert(i, name.clone());
    }

    let mut bold_family: Vec<String> = Vec::new();
    if let Some(data) = load("seguisb.ttf").or_else(|| load("segoeuib.ttf")) {
        fonts.font_data.insert("segoe_bold".into(), data);
        bold_family.push("segoe_bold".into());
    }
    bold_family.extend(fonts.families.get(&FontFamily::Proportional).cloned().unwrap_or_default());
    fonts.families.insert(bold(), bold_family);

    ctx.set_fonts(fonts);
}
