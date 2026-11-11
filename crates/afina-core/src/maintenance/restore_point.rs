//! System Restore point via PowerShell's `Checkpoint-Computer`.

use afina_platform::process;

use crate::{Error, Result};

/// Windows allows one restore point per 24 h by default; the script lifts that
/// limit only for this call and puts the original setting back afterwards.
const SCRIPT: &str = r#"
$ErrorActionPreference = 'Stop'
$k = 'HKLM:\SOFTWARE\Microsoft\Windows NT\CurrentVersion\SystemRestore'
$old = (Get-ItemProperty -Path $k -Name SystemRestorePointCreationFrequency -ErrorAction SilentlyContinue).SystemRestorePointCreationFrequency
Set-ItemProperty -Path $k -Name SystemRestorePointCreationFrequency -Value 0 -Type DWord
try {
  Enable-ComputerRestore -Drive "$env:SystemDrive\"
  Checkpoint-Computer -Description 'Afina - antes dos ajustes' -RestorePointType MODIFY_SETTINGS
  'AFINA_OK'
} finally {
  if ($null -eq $old) { Remove-ItemProperty -Path $k -Name SystemRestorePointCreationFrequency -ErrorAction SilentlyContinue }
  else { Set-ItemProperty -Path $k -Name SystemRestorePointCreationFrequency -Value $old -Type DWord }
}
"#;

pub(super) fn create() -> Result<String> {
    let what = "ponto de restauração";
    let out = process::run_hidden(
        "powershell.exe",
        &["-NoProfile", "-NonInteractive", "-ExecutionPolicy", "Bypass", "-Command", SCRIPT],
    )
    .map_err(|e| Error::io(what, e))?;
    if out.success && out.text.contains("AFINA_OK") {
        Ok("Ponto de restauração criado. Se algo der errado, use \"Recuperação\" no Painel de Controle.".into())
    } else {
        Err(Error::command(what, out.last_line()))
    }
}
