//! Activity log: shown in the footer and appended to `%LOCALAPPDATA%\Afina\afina.log`.

use std::collections::VecDeque;
use std::io::Write;
use std::path::PathBuf;

use afina_platform::time::now_hms;

const MAX_LINES: usize = 300;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Level {
    Info,
    Ok,
    Err,
}

#[derive(Debug, Clone)]
pub struct LogLine {
    pub time: String,
    pub level: Level,
    pub text: String,
}

#[derive(Debug)]
pub struct ActivityLog {
    lines: VecDeque<LogLine>,
    file: Option<PathBuf>,
}

impl ActivityLog {
    /// `file: None` keeps the log in memory only (used by tests).
    pub fn new(file: Option<PathBuf>) -> Self {
        ActivityLog { lines: VecDeque::new(), file }
    }

    pub fn push(&mut self, level: Level, text: impl Into<String>) {
        let line = LogLine { time: now_hms(), level, text: text.into() };
        self.append_to_file(&line);
        self.lines.push_back(line);
        if self.lines.len() > MAX_LINES {
            self.lines.pop_front();
        }
    }

    pub fn info(&mut self, text: impl Into<String>) {
        self.push(Level::Info, text);
    }

    pub fn ok(&mut self, text: impl Into<String>) {
        self.push(Level::Ok, text);
    }

    pub fn err(&mut self, text: impl Into<String>) {
        self.push(Level::Err, text);
    }

    pub fn last(&self) -> Option<&LogLine> {
        self.lines.back()
    }

    pub fn iter(&self) -> impl Iterator<Item = &LogLine> {
        self.lines.iter()
    }

    fn append_to_file(&self, line: &LogLine) {
        let Some(path) = &self.file else { return };
        if let Some(dir) = path.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        let tag = match line.level {
            Level::Info => "INFO",
            Level::Ok => "OK  ",
            Level::Err => "ERRO",
        };
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
            let _ = writeln!(f, "{} {tag} {}", line.time, line.text);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_order_and_caps_size() {
        let mut log = ActivityLog::new(None);
        for i in 0..MAX_LINES + 5 {
            log.info(format!("linha {i}"));
        }
        log.err("falhou");
        assert_eq!(log.iter().count(), MAX_LINES);
        assert_eq!(log.last().map(|l| (l.level, l.text.as_str())), Some((Level::Err, "falhou")));
    }
}
