//! Telling Windows that a setting changed, so it applies without signing out.

use std::ffi::c_void;

use windows_sys::Win32::UI::WindowsAndMessaging::{
    HWND_BROADCAST, SMTO_ABORTIFHUNG, SPI_SETMOUSE, SPIF_SENDCHANGE, SPIF_UPDATEINIFILE, SendMessageTimeoutW,
    SystemParametersInfoW, WM_SETTINGCHANGE,
};

use crate::registry::{self, Hive};
use crate::wide;

/// Re-applies the mouse acceleration values currently in the registry.
pub fn refresh_mouse_settings() {
    let path = r"Control Panel\Mouse";
    let get = |name: &str, default: i32| -> i32 {
        registry::read_string(Hive::Hkcu, path, name).and_then(|s| s.trim().parse().ok()).unwrap_or(default)
    };
    // SPI_SETMOUSE expects [threshold1, threshold2, acceleration].
    let mut params: [i32; 3] = [get("MouseThreshold1", 6), get("MouseThreshold2", 10), get("MouseSpeed", 1)];
    // SAFETY: `params` is a valid 3-int array, as SPI_SETMOUSE requires, alive for the call.
    unsafe {
        SystemParametersInfoW(
            SPI_SETMOUSE,
            0,
            params.as_mut_ptr() as *mut c_void,
            SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
        );
    }
}

/// Broadcasts a theme change (transparency, animations) to Explorer and apps.
pub fn broadcast_theme_change() {
    let area = wide("ImmersiveColorSet");
    let mut result: usize = 0;
    // SAFETY: `area` is a null-terminated UTF-16 string alive for the call; `result` is a valid out pointer.
    unsafe {
        SendMessageTimeoutW(
            HWND_BROADCAST,
            WM_SETTINGCHANGE,
            0,
            area.as_ptr() as isize,
            SMTO_ABORTIFHUNG,
            1000,
            &mut result,
        );
    }
}
