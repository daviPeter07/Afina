//! The window frame around pages: header (top), sidebar (left), footer (bottom).
//! Same contract as pages: draw from `&AppState`, emit `Action`s.

mod footer;
mod header;
mod sidebar;

pub use footer::footer;
pub use header::header;
pub use sidebar::sidebar;
