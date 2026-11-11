//! Error type of the domain layer. Messages are user-facing (pt-BR).

use std::{fmt, io};

#[derive(Debug)]
pub enum Error {
    /// Registry or file-system failure while handling `what`.
    Io { what: String, source: io::Error },
    /// A Windows tool (powercfg, w32tm, ...) reported failure.
    Command { what: String, detail: String },
    /// The feature is not available on this PC.
    Unsupported(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub(crate) fn io(what: impl Into<String>, source: io::Error) -> Self {
        Error::Io { what: what.into(), source }
    }

    pub(crate) fn command(what: impl Into<String>, detail: impl Into<String>) -> Self {
        Error::Command { what: what.into(), detail: detail.into() }
    }

    /// True when the failure is due to missing administrator rights.
    pub fn is_access_denied(&self) -> bool {
        matches!(self, Error::Io { source, .. } if source.kind() == io::ErrorKind::PermissionDenied)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io { what, source } if source.kind() == io::ErrorKind::PermissionDenied => {
                write!(f, "{what}: acesso negado — execute o Afina como administrador")
            }
            Error::Io { what, source } => write!(f, "{what}: {source}"),
            Error::Command { what, detail } => write!(f, "{what}: {detail}"),
            Error::Unsupported(msg) => f.write_str(msg),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io { source, .. } => Some(source),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn access_denied_message_tells_user_what_to_do() {
        let e = Error::io("HwSchMode", io::Error::from(io::ErrorKind::PermissionDenied));
        assert!(e.is_access_denied());
        assert!(e.to_string().contains("administrador"));
        let e = Error::command("powercfg", "falhou");
        assert!(!e.is_access_denied());
        assert_eq!(e.to_string(), "powercfg: falhou");
    }
}
