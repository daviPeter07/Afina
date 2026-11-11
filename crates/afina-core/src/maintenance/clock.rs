//! Clock health. A wrong clock makes online games reject logins (session tokens
//! look expired) — e.g. VALORANT's VAL 7.

use afina_platform::process;

use crate::{Error, Result};

/// Turns on automatic sync and forces a resync now. Needs admin.
pub(super) fn sync_clock() -> Result<String> {
    // Best effort: the service may already be configured/running.
    let _ = process::run_hidden("sc", &["config", "w32time", "start=", "auto"]);
    let _ = process::run_hidden("net", &["start", "w32time"]);
    let _ = process::run_hidden("w32tm", &["/config", "/update"]);
    let what = "w32tm /resync";
    let out = process::run_hidden("w32tm", &["/resync", "/force"]).map_err(|e| Error::io(what, e))?;
    if !out.success {
        return Err(Error::command("o Windows não conseguiu sincronizar", out.last_line()));
    }
    match clock_offset() {
        Ok(off) if off.abs() <= 5.0 => Ok("Relógio sincronizado. Sincronização automática ligada.".into()),
        Ok(off) => Err(Error::command(
            "relógio ainda fora da hora certa",
            format!("diferença de {:.0} s — confira o fuso horário em Configurações > Hora e idioma", off.abs()),
        )),
        Err(_) => Ok("Relógio sincronizado. Sincronização automática ligada.".into()),
    }
}

/// Difference in seconds between this PC and `time.windows.com`. No admin needed.
/// Only the magnitude is meaningful to callers (the sign convention of w32tm is
/// "server relative to local").
pub fn clock_offset() -> Result<f64> {
    let what = "w32tm /stripchart";
    let out = process::run_hidden("w32tm", &["/stripchart", "/computer:time.windows.com", "/samples:1", "/dataonly"])
        .map_err(|e| Error::io(what, e))?;
    parse_stripchart(&out.text).ok_or_else(|| Error::command(what, "sem resposta do servidor de horário"))
}

/// Parses the offset from `w32tm /stripchart /dataonly`, e.g.
/// `04:00:00, -14400.0012345s` (en-US) or `04:00:00, +00,0123456s` (decimal comma).
pub fn parse_stripchart(output: &str) -> Option<f64> {
    output.lines().rev().find_map(|line| {
        let (_, rest) = line.trim().rsplit_once(", ")?;
        let number = rest.trim().strip_suffix('s')?;
        number.replace(',', ".").parse::<f64>().ok()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_both_decimal_separators() {
        let en = "Tracking time.windows.com [20.101.57.9:123].\nCollecting 1 samples.\nThe current time is 11/11/2026 4:00:00 AM.\n04:00:00, -14400.0012345s\n";
        assert!((parse_stripchart(en).unwrap_or_default() + 14400.0012345).abs() < 1e-6);
        let pt = "Acompanhando time.windows.com.\nColetando 1 amostras.\nA hora atual é 11/11/2026 04:00:00.\n04:00:00, +00,0123456s\n";
        assert!((parse_stripchart(pt).unwrap_or_default() - 0.0123456).abs() < 1e-6);
    }

    #[test]
    fn errors_and_empty_output_give_none() {
        assert_eq!(parse_stripchart("04:00:00, erro: 0x800705B4"), None);
        assert_eq!(parse_stripchart(""), None);
    }
}
