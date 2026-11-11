//! `afina-core` — everything Afina *knows* and *does*, with no UI.
//!
//! * [`catalog`]  — the declarative list of tweaks (what exists).
//! * [`engine`]   — reads state, applies and reverts tweaks with backup (how it is done).
//! * [`backup`]   — persistent record of original values.
//! * [`startup`]  — startup apps (Task Manager's `StartupApproved` mechanism).
//! * [`maintenance`] — one-shot actions: restore point, temp cleanup, clock, DNS.
//!
//! Windows access goes through `afina-platform`; this crate contains no `unsafe`.
#![forbid(unsafe_code)]

pub mod backup;
pub mod catalog;
pub mod engine;
pub mod error;
pub mod maintenance;
pub mod paths;
pub mod startup;
pub mod tweak;

pub use error::{Error, Result};
