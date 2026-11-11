# Afina — guia para o Claude

App desktop open source (Rust + egui, só Windows 10/11) que ajusta o Windows para jogos.
Um único `.exe`, sem instalação, sem rede (exceto `w32tm` checando a hora), sem telemetria.
Licença MIT. Arquitetura para humanos: `docs/ARCHITECTURE.md`.

## Comandos

```powershell
cargo build --workspace                                # debug
cargo run -p afina-app                                 # abre o app (binário "afina")
cargo test --workspace                                 # 29+ testes; usam só HKCU\Software\AfinaTest
cargo clippy --workspace --all-targets -- -D warnings  # deve passar sem nenhum aviso
cargo fmt --all                                        # max_width = 120 (rustfmt.toml)
cargo build --release                                  # target\release\afina.exe (~4,4 MB)
$env:AFINA_PAGINA = "jogos"; cargo run -p afina-app    # abre direto numa aba (inicio|jogos|sistema|visual|inicializacao|ferramentas|restaurar|sobre)
```

Toolchain local, PATH e particularidades da máquina: ver `CLAUDE.local.md` (não versionado), se existir.

## Mapa do workspace

Manifesto virtual na raiz; crates planos em `crates/` (nome do crate = nome da pasta).
As versões das dependências ficam SÓ em `[workspace.dependencies]` do `Cargo.toml` raiz.

| Crate | Papel | Pode depender de |
|---|---|---|
| `afina-platform` | Windows "cru": registro, processos, UAC, sysinfo, notificações. **Único lugar com `unsafe`** | nada interno |
| `afina-core` | Domínio: `tweak` (modelo), `catalog` (dados), `engine` (aplicar/desfazer), `backup`, `startup`, `maintenance` | `afina-platform` |
| `afina-app` | UI egui (bin `afina`): `pages`, `shell`, `components`, `theme`, `controller` | `afina-core`, `afina-platform` |

Regras de fronteira (não quebre):
- `afina-core` e `afina-app` têm `#![forbid(unsafe_code)]`. FFI nova vai para `afina-platform` atrás de função segura com comentário `// SAFETY:`.
- `afina-platform` não conhece "ajustes"; `afina-core` não conhece egui; `afina-app` não escreve no registro direto (passa por `afina-core`).
- Lógica de domínio NUNCA em página/componente.

## Invariantes do produto (o porquê do projeto)

1. **Estado real**: o estado de um ajuste é sempre LIDO do Windows (`engine::is_on`), nunca presumido.
2. **Reversível**: antes da 1ª mudança, os originais vão para `%LOCALAPPDATA%\Afina\backup.json`; o primeiro snapshot nunca é sobrescrito.
3. **Sem placebo**: só ajustes com efeito documentado. Recusar Nagle/TcpAckFrequency, NetworkThrottlingIndex, "limpar RAM", desligar serviços em massa, apagar cache de shaders, desligar Defender/Update (lista em `README.md`).
4. **Nada residente**: sem serviço, sem tarefa agendada, sem processo em segundo plano depois que o app fecha.
5. **Transparência**: cada ajuste explica ganho e custo; risco `Avancado` pede confirmação e nunca é recomendado.

## Convenções

- Textos de UI, mensagens de erro e docs de usuário: **português do Brasil**. Identificadores e comentários de código: **inglês**.
- Erros de domínio: `afina_core::Error` (mensagem já pronta para o usuário). Sem `unwrap()`/`expect()` fora de testes (lint `clippy::unwrap_used`); testes usam `#[allow(clippy::unwrap_used)]`.
- Cores só via `theme::palette`; widgets reutilizáveis em `components/` (sem estado, sem `Action`).
- Ids de ajuste (`Tweak::id`) são gravados no backup: **nunca renomeie um id existente**.

## Fluxo da UI (mão única)

`pages/*` e `shell/*` recebem `&AppState` e empurram `Action` → `AfinaApp::dispatch` (`controller.rs`) chama `afina-core` e atualiza `AppState` → próximo frame redesenha.
Trabalho lento (ponto de restauração, limpeza, checagem do relógio) roda em thread via `jobs.rs`; a UI só faz `poll`.

## Receitas

- **Novo ajuste**: entrada em `crates/afina-core/src/catalog/<categoria>.rs`. Nenhuma outra mudança é necessária; a UI lista sozinha. Rode o teste `catalog_is_consistent`. Detalhes nas regras do core.
- **Novo tipo de ajuste** (além de Registry/DxGlobalFlag/PowerPlan): variante em `tweak::Kind` + módulo em `engine/` com `is_on`/`turn_on`/`turn_off`/`restore` + braços em `engine/mod.rs`.
- **Nova ferramenta**: variante em `maintenance::Tool` (+ `ALL`, `label`, `description`, `needs_admin`, `run`) e um arquivo em `maintenance/`.
- **Nova página**: variante em `state::Page` (+ `ALL`, `label`, `from_name`) e arquivo em `pages/` com `pub fn show(ui, &AppState, &mut Vec<Action>)`.
- **Nova interação**: variante em `action::Action` + braço em `controller.rs`.

## Testes

- Testes que tocam o registro usam SOMENTE chaves sob `HKCU\Software\AfinaTest\<nome-único>` e limpam com `registry::delete_test_tree` (que entra em pânico fora dessa raiz). Cada teste usa a própria chave (rodam em paralelo).
- Nunca escreva teste que altere configuração real do Windows, rode `powercfg`/`w32tm /resync` ou exija admin.
- Teste de integração do motor: `crates/afina-core/tests/engine_registry.rs`.

## Definição de pronto

1. `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings` e `cargo test --workspace` passando.
2. Mudou UI? Gere o release e confira a tela (ex.: `AFINA_PAGINA`) — o app deve ficar com ~0% de CPU parado.
3. Mudou comportamento visível? Atualize `README.md` (tabela de ajustes) e, se mudou estrutura, `docs/ARCHITECTURE.md` e as regras em `.claude/rules/`.

## Pegadinhas conhecidas

- **egui/eframe 0.37**: o trait é `App::ui(&mut self, ui: &mut Ui, frame)` (não `update`); painéis são `egui::Panel::top/left/bottom(id).show(ui, ..)`; `Id::new` está obsoleto (use `Id::unique`). Na dúvida, leia o código em `~/.cargo/registry/src/*/egui-0.37.0/`.
- **winreg 0.56**: `HKCU`/`HKLM` são `&'static RegKey`; `RegValue.bytes` é `Cow<[u8]>`.
- Saídas de `powercfg`/`w32tm` são traduzidas (pt-BR): extraia só GUIDs/números, nunca palavras; aceite vírgula decimal.
- Build debug consome CPU em animações; meça desempenho só no release.
- `ProductName` do registro diz "Windows 10" no Windows 11 — use o número do build (≥ 22000).

## Regras por módulo

Carregadas automaticamente só quando você mexe nos arquivos do módulo:
`.claude/rules/platform/CLAUDE.md`, `.claude/rules/core/CLAUDE.md`, `.claude/rules/app/CLAUDE.md`.
