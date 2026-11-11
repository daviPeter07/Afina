//! One-shot maintenance actions. They may take seconds, so the UI runs them on
//! a background thread. Each returns a user-facing success message.

mod clock;
mod dns;
mod restore_point;
mod temp;

pub use clock::{clock_offset, parse_stripchart};
pub use temp::clean_dir;

use crate::Result;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tool {
    RestorePoint,
    CleanTemp,
    SyncClock,
    FlushDns,
}

impl Tool {
    pub const ALL: [Tool; 4] = [Tool::RestorePoint, Tool::CleanTemp, Tool::SyncClock, Tool::FlushDns];

    pub fn label(self) -> &'static str {
        match self {
            Tool::RestorePoint => "Criar ponto de restauração",
            Tool::CleanTemp => "Limpar arquivos temporários",
            Tool::SyncClock => "Acertar o relógio",
            Tool::FlushDns => "Limpar cache de DNS",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Tool::RestorePoint => "Uma \"foto\" do sistema para voltar atrás se algo der errado. Leva até um minuto.",
            Tool::CleanTemp => {
                "Apaga arquivos temporários com mais de 24 horas. Não mexe em cache de shaders dos jogos (isso causaria travadas)."
            }
            Tool::SyncClock => {
                "Liga a sincronização automática e acerta a hora. Relógio errado causa falhas de login em jogos online."
            }
            Tool::FlushDns => "Resolve sites ou servidores de jogo que \"não conectam\" depois de mudar de rede.",
        }
    }

    pub fn needs_admin(self) -> bool {
        matches!(self, Tool::RestorePoint | Tool::SyncClock)
    }

    /// Blocking. Call from a worker thread.
    pub fn run(self) -> Result<String> {
        match self {
            Tool::RestorePoint => restore_point::create(),
            Tool::CleanTemp => temp::clean_temp(),
            Tool::SyncClock => clock::sync_clock(),
            Tool::FlushDns => dns::flush(),
        }
    }
}
