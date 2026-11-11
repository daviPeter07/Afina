//! Administrator rights (UAC).

use std::io;

use windows_sys::Win32::UI::Shell::{IsUserAnAdmin, ShellExecuteW};
use windows_sys::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;

use crate::wide;

/// Is the current process elevated?
pub fn is_admin() -> bool {
    // SAFETY: no arguments, no preconditions.
    unsafe { IsUserAnAdmin() != 0 }
}

/// Starts an elevated copy of the current executable (shows the UAC prompt).
/// Returns an error if the user declined or the launch failed.
pub fn relaunch_as_admin() -> io::Result<()> {
    let exe = std::env::current_exe()?;
    let exe_w = wide(&exe.to_string_lossy());
    let verb = wide("runas");
    // SAFETY: all pointers are valid, null-terminated UTF-16 buffers that outlive the call.
    let ret = unsafe {
        ShellExecuteW(
            std::ptr::null_mut(),
            verb.as_ptr(),
            exe_w.as_ptr(),
            std::ptr::null(),
            std::ptr::null(),
            SW_SHOWNORMAL,
        )
    };
    // ShellExecute returns a value > 32 on success.
    if (ret as isize) > 32 {
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::PermissionDenied, "elevação recusada"))
    }
}
