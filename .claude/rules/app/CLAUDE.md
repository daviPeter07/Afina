---
paths:
  - "crates/afina-app/**"
---

# Módulo `afina-app` — interface (egui 0.37)

Binário `afina`. Só apresentação e orquestração; regras de negócio ficam no `afina-core`.

## Arquitetura: fluxo de mão única

```
pages/*, shell/*, dialogs.rs  --(&AppState)-->  desenham  --push-->  Vec<Action>
app.rs (frame): poll_background → desenha tudo coletando ações → dispatch(cada ação)
controller.rs: ÚNICO lugar que muta AppState e chama operações do afina-core com efeito
```

| Arquivo/pasta | Responsabilidade |
|---|---|
| `main.rs` | Janela (tamanho, ícone, título) e declaração dos módulos |
| `app.rs` | `AfinaApp` (estado + workers), layout dos painéis, loop do frame, `refresh_*` |
| `action.rs` | `enum Action` — tudo que o usuário pode pedir |
| `controller.rs` | `AfinaApp::dispatch` + `poll_background`; chama `engine`, `startup`, `jobs` |
| `state.rs` | `AppState`, `Page` (`ALL`, `label`, `from_name`), `Confirm`, `ClockStatus` |
| `jobs.rs` | `JobRunner` (1 ferramenta por vez em thread) e `ClockProbe`; nunca bloquear a UI |
| `activity.rs` | `ActivityLog` (memória + `afina.log`) |
| `dialogs.rs` | Modal de confirmação (`AppState::confirm`); Esc/clique fora = cancelar |
| `shell/` | `header`, `sidebar`, `footer` |
| `pages/` | Uma página por arquivo: `home`, `tweaks` (Jogos/Sistema/Visual), `startup`, `tools`, `restore`, `about` |
| `components/` | Widgets sem estado: `card`, `badge`/`risk_badge`, `toggle`, `primary_button`/`danger_button`, `page_title`/`section_title`/`info_row` |
| `theme/` | `palette.rs` (TODAS as cores), `fonts.rs` (Segoe UI + Segoe UI Symbol), `apply()` |
| `icon.rs` | Ícone desenhado em código |

## Regras

- Página/shell: assinatura `fn show(ui: &mut Ui, state: &AppState, out: &mut Vec<Action>)`. Nunca mutar estado nem chamar `engine`/`registry` de dentro de uma view.
- Componente: recebe valores simples, devolve `Response`/nada; não conhece `AppState` nem `Action`.
- Cores só de `theme::palette` (nunca `Color32::from_rgb` numa view). Títulos em negrito: `theme::bold()`.
- Ícones/símbolos: use caracteres que a Segoe UI Symbol tem (✔ ⚡ ⟳ 🛡 🎮 🔧...). Teste visualmente: glifo ausente vira "▯".
- Texto ao usuário em pt-BR; números decimais com vírgula (`decimal_br` em `pages/home.rs`).
- Ajustes que exigem admin: desabilitar o controle com `add_enabled(..)` + `on_disabled_hover_text("Requer administrador...")`.
- Risco `Avancado` ligando ⇒ o controller abre `Confirm::EnableAdvanced` (não pule isso).
- Algo lento (> ~50 ms)? Vira `maintenance::Tool` executado pelo `JobRunner`, nunca no thread da UI.
- Repaint contínuo só enquanto há job/probe rodando (`request_repaint_after`); parado, o app deve ficar em ~0% de CPU.

## API do egui 0.37 (mudou bastante)

- `impl eframe::App { fn ui(&mut self, ui: &mut Ui, frame) }` — não existe mais `update(ctx, ..)`.
- `egui::Panel::top/bottom/left(id).frame(..).exact_size(..).show(ui, ..)`; `CentralPanel::default().show(ui, ..)`.
- `egui::Modal::new(Id::unique(..)).show(ctx, ..)` → `.should_close()`. `Id::new` está obsoleto.
- `painter.rect(rect, CornerRadius, fill, stroke, StrokeKind::Inside)`; `Margin::symmetric(i8, i8)`.

## Verificação visual

`$env:AFINA_PAGINA="jogos"` (ou outra aba) + `cargo run -p afina-app` abre direto na aba. Confira no build release.
