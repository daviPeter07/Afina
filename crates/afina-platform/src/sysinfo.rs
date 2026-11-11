//! Hardware / OS summary shown on the home page. Read-only, no admin needed.

use windows_sys::Win32::System::SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX};

use crate::registry::{self, Hive};

#[derive(Clone, Debug, Default)]
pub struct SysInfo {
    pub os: String,
    pub cpu: String,
    pub gpus: Vec<String>,
    pub ram_gb: f64,
}

pub fn sys_info() -> SysInfo {
    SysInfo { os: os_name(), cpu: cpu_name(), gpus: gpu_names(), ram_gb: total_ram_gb() }
}

fn os_name() -> String {
    let nt = r"SOFTWARE\Microsoft\Windows NT\CurrentVersion";
    let build: u32 =
        registry::read_string(Hive::Hklm, nt, "CurrentBuildNumber").and_then(|s| s.parse().ok()).unwrap_or(0);
    // `ProductName` still says "Windows 10" on Windows 11, so decide by build number.
    let name = if build >= 22000 { "Windows 11" } else { "Windows 10" };
    let display = registry::read_string(Hive::Hklm, nt, "DisplayVersion").unwrap_or_default();
    format!("{name} {display} (build {build})").replace("  ", " ")
}

fn cpu_name() -> String {
    registry::read_string(Hive::Hklm, r"HARDWARE\DESCRIPTION\System\CentralProcessor\0", "ProcessorNameString")
        .map(|s| s.split_whitespace().collect::<Vec<_>>().join(" "))
        .unwrap_or_else(|| "CPU desconhecida".into())
}

/// Display adapters from the driver class key, minus virtual/basic adapters.
fn gpu_names() -> Vec<String> {
    let class = r"SYSTEM\CurrentControlSet\Control\Class\{4d36e968-e325-11ce-bfc1-08002be10318}";
    let mut gpus: Vec<String> = Vec::new();
    for sub in registry::list_subkeys(Hive::Hklm, class) {
        if !sub.chars().all(|c| c.is_ascii_digit()) {
            continue;
        }
        let Some(desc) = registry::read_string(Hive::Hklm, &format!(r"{class}\{sub}"), "DriverDesc") else {
            continue;
        };
        let lower = desc.to_lowercase();
        let is_virtual =
            ["basic display", "basic render", "remote display", "virtual"].iter().any(|k| lower.contains(k));
        if !is_virtual && !gpus.contains(&desc) {
            gpus.push(desc);
        }
    }
    gpus
}

fn total_ram_gb() -> f64 {
    // SAFETY: MEMORYSTATUSEX is plain data; zeroed is valid before setting dwLength.
    let mut mem: MEMORYSTATUSEX = unsafe { std::mem::zeroed() };
    mem.dwLength = std::mem::size_of::<MEMORYSTATUSEX>() as u32;
    // SAFETY: `mem` is a valid, writable struct with dwLength set, as the API requires.
    if unsafe { GlobalMemoryStatusEx(&mut mem) } != 0 {
        mem.ullTotalPhys as f64 / (1024.0 * 1024.0 * 1024.0)
    } else {
        0.0
    }
}
