//! Background work. Slow operations (restore point, cleanup, clock check) run
//! on a worker thread; the UI polls once per frame and never blocks.

use std::sync::mpsc::{Receiver, TryRecvError, channel};

use afina_core::maintenance::{self, Tool};

/// What to do once a job finishes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum After {
    Nothing,
    /// "Aplicar recomendados" with "criar ponto de restauração antes".
    ApplyRecommended,
}

#[derive(Debug)]
pub struct Finished {
    pub tool: Tool,
    pub after: After,
    pub result: afina_core::Result<String>,
}

#[derive(Debug)]
struct Running {
    tool: Tool,
    after: After,
    rx: Receiver<afina_core::Result<String>>,
}

/// Runs at most one maintenance tool at a time.
#[derive(Debug, Default)]
pub struct JobRunner {
    running: Option<Running>,
}

impl JobRunner {
    pub fn busy(&self) -> Option<Tool> {
        self.running.as_ref().map(|r| r.tool)
    }

    /// Returns false (and does nothing) if a job is already running.
    pub fn start(&mut self, tool: Tool, after: After) -> bool {
        if self.running.is_some() {
            return false;
        }
        let (tx, rx) = channel();
        std::thread::spawn(move || {
            let _ = tx.send(tool.run());
        });
        self.running = Some(Running { tool, after, rx });
        true
    }

    pub fn poll(&mut self) -> Option<Finished> {
        let running = self.running.as_ref()?;
        let result = match running.rx.try_recv() {
            Ok(r) => r,
            Err(TryRecvError::Empty) => return None,
            Err(TryRecvError::Disconnected) => {
                Err(afina_core::Error::Unsupported("a tarefa terminou sem responder".into()))
            }
        };
        let Running { tool, after, .. } = self.running.take()?;
        Some(Finished { tool, after, result })
    }
}

/// Compares the PC clock with internet time in the background.
#[derive(Debug)]
pub struct ClockProbe {
    rx: Option<Receiver<afina_core::Result<f64>>>,
}

impl ClockProbe {
    pub fn start() -> Self {
        let (tx, rx) = channel();
        std::thread::spawn(move || {
            let _ = tx.send(maintenance::clock_offset());
        });
        ClockProbe { rx: Some(rx) }
    }

    pub fn is_running(&self) -> bool {
        self.rx.is_some()
    }

    pub fn poll(&mut self) -> Option<afina_core::Result<f64>> {
        let result = match self.rx.as_ref()?.try_recv() {
            Ok(r) => r,
            Err(TryRecvError::Empty) => return None,
            Err(TryRecvError::Disconnected) => Err(afina_core::Error::Unsupported("verificação interrompida".into())),
        };
        self.rx = None;
        Some(result)
    }
}
