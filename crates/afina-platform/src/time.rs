//! Local wall-clock time without pulling in a date/time crate.

use windows_sys::Win32::Foundation::SYSTEMTIME;
use windows_sys::Win32::System::SystemInformation::GetLocalTime;

/// Current local time as `HH:MM:SS`.
pub fn now_hms() -> String {
    // SAFETY: SYSTEMTIME is plain data; zeroed is a valid value and GetLocalTime fills it.
    let mut st: SYSTEMTIME = unsafe { std::mem::zeroed() };
    // SAFETY: `st` is a valid, writable SYSTEMTIME.
    unsafe { GetLocalTime(&mut st) };
    format!("{:02}:{:02}:{:02}", st.wHour, st.wMinute, st.wSecond)
}
