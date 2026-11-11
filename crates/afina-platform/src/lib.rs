//! `afina-platform` — the only crate that talks to Windows directly.
//!
//! Every `unsafe` block and every FFI call of the project lives here, behind
//! small safe functions. Nothing in this crate knows what a "tweak" is.

pub mod elevation;
pub mod notify;
pub mod process;
pub mod registry;
pub mod sysinfo;
pub mod time;

/// Null-terminated UTF-16 string for Win32 calls.
pub(crate) fn wide(s: &str) -> Vec<u16> {
    use std::os::windows::ffi::OsStrExt;
    std::ffi::OsStr::new(s).encode_wide().chain(std::iter::once(0)).collect()
}
