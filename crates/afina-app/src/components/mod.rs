//! Reusable, stateless widgets. Components never emit actions or read
//! `AppState`: they take plain values and return `egui::Response`s.

mod badge;
mod buttons;
mod card;
mod text;
mod toggle;

pub use badge::{badge, risk_badge};
pub use buttons::{danger_button, primary_button};
pub use card::card;
pub use text::{info_row, page_title, section_title};
pub use toggle::toggle;
