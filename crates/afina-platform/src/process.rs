//! Running Windows tools (powercfg, w32tm, ...) without flashing console windows.

use std::io;
use std::os::windows::process::CommandExt;
use std::process::Command;

const CREATE_NO_WINDOW: u32 = 0x0800_0000;

/// Result of a finished command.
#[derive(Debug, Clone)]
pub struct Output {
    pub success: bool,
    /// stdout followed by stderr, lossily decoded.
    pub text: String,
}

impl Output {
    /// Last non-empty line — usually the most useful bit of an error message.
    pub fn last_line(&self) -> &str {
        self.text.lines().map(str::trim).rfind(|l| !l.is_empty()).unwrap_or("")
    }
}

/// Runs a console program hidden and waits for it.
pub fn run_hidden(program: &str, args: &[&str]) -> io::Result<Output> {
    let out = Command::new(program).args(args).creation_flags(CREATE_NO_WINDOW).output()?;
    let mut text = String::from_utf8_lossy(&out.stdout).into_owned();
    text.push_str(&String::from_utf8_lossy(&out.stderr));
    Ok(Output { success: out.status.success(), text })
}

/// Opens a URI (e.g. `ms-settings:gaming-gamemode`) or a folder with the shell.
pub fn open(target: &str) -> io::Result<()> {
    Command::new("explorer.exe").arg(target).spawn().map(|_| ())
}

/// Restarts the computer immediately.
pub fn reboot_now() -> io::Result<()> {
    run_hidden("shutdown", &["/r", "/t", "0"]).map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn last_line_skips_blank_lines() {
        let o = Output { success: false, text: "linha 1\nerro final\n\n  \n".into() };
        assert_eq!(o.last_line(), "erro final");
        assert_eq!(Output { success: true, text: String::new() }.last_line(), "");
    }
}
