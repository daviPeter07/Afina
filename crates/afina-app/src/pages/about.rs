//! Sobre: version, principles, references.

use eframe::egui::{RichText, Ui};

use crate::action::Action;
use crate::components::{card, page_title, section_title};
use crate::state::AppState;
use crate::theme::palette::MUTED;

const PRINCIPLES: [&str; 4] = [
    "Só ajustes com efeito documentado — nada de \"truques\" placebo de rede ou de registro.",
    "Estado real: cada botão mostra o que está de fato configurado no Windows.",
    "Tudo reversível: valores originais salvos antes de qualquer mudança.",
    "Sem internet, sem telemetria, sem instalação: um único .exe.",
];

const REFERENCES: [&str; 3] = [
    "Microsoft — \"Options to optimize gaming performance in Windows 11\" (HVCI/VMP)",
    "WinUtil (Chris Titus Tech, MIT) — valores padrão de Modo de Jogo, mouse e apps em segundo plano",
    "Documentação do Windows: powercfg, w32tm, StartupApproved",
];

pub fn show(ui: &mut Ui, _state: &AppState, _out: &mut Vec<Action>) {
    let version = format!("Versão {} • código aberto (licença MIT)", env!("CARGO_PKG_VERSION"));
    page_title(ui, "Sobre o Afina", &version);
    list_card(ui, "Princípios", &PRINCIPLES);
    list_card(ui, "Referências", &REFERENCES);
}

fn list_card(ui: &mut Ui, title: &str, lines: &[&str]) {
    card(ui, |ui| {
        section_title(ui, title);
        for line in lines {
            ui.label(RichText::new(format!("•  {line}")).color(MUTED));
        }
    });
}
