---
paths:
  - "crates/afina-platform/**"
---

# Módulo `afina-platform` — Windows cru

Camada fina e segura sobre APIs do Windows. É o ÚNICO crate com `unsafe` e com
dependência de `windows-sys`/`winreg`. Não conhece ajustes, backup nem UI.

## Arquivos

| Arquivo | Conteúdo |
|---|---|
| `registry.rs` | `Hive` (HKCU/HKLM), `RawValue` (tipo + bytes), `read`/`read_opt`/`read_string`/`write`/`delete`/`put`/`list_values`/`list_subkeys`, `delete_test_tree` |
| `process.rs` | `run_hidden` (sem janela de console, `CREATE_NO_WINDOW`), `Output::last_line`, `open` (URI/pasta), `reboot_now` |
| `elevation.rs` | `is_admin`, `relaunch_as_admin` (UAC via `ShellExecuteW "runas"`) |
| `sysinfo.rs` | `SysInfo` (SO, CPU, GPUs, RAM) — só leitura, sem admin |
| `notify.rs` | `refresh_mouse_settings` (`SPI_SETMOUSE`), `broadcast_theme_change` (`WM_SETTINGCHANGE`) |
| `time.rs` | `now_hms` (hora local sem crate de data) |
| `lib.rs` | `wide()` — string UTF-16 terminada em zero para Win32 |

## Regras

- Todo `unsafe` fica num bloco mínimo com comentário `// SAFETY:` explicando por que os ponteiros/tamanhos são válidos.
- Exponha funções **seguras** e pequenas; retorne `std::io::Result` (o core converte para `afina_core::Error`).
- "Não encontrado" no registro NÃO é erro: `read` → `Ok(None)`, `delete` idempotente.
- Valores do registro sempre como `RawValue` bruto para que backup/restauração sejam byte a byte (qualquer tipo, inclusive REG_BINARY/MULTI_SZ).
- Comandos externos: sempre `run_hidden` (nunca `Command` direto em outro crate). A saída é traduzida pelo Windows — quem chama extrai GUIDs/números, não palavras.
- Feature nova do `windows-sys`? Adicione em `[workspace.dependencies]` no `Cargo.toml` raiz, não aqui.
- Assinaturas do `windows-sys` 0.61: `HWND`/`HINSTANCE` são `*mut c_void` (use `std::ptr::null_mut()`); `BOOL` é `i32`.

## Testes

- Só em `HKCU\Software\AfinaTest\<nome-único>`; limpar com `delete_test_tree` (pânico fora de `TEST_ROOT` — é a trava de segurança, não remova).
- Nada que exija admin ou mude configuração real.
