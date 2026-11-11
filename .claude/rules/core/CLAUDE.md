---
paths:
  - "crates/afina-core/**"
---

# Módulo `afina-core` — domínio

Tudo o que o Afina sabe e faz, sem UI e sem `unsafe` (`#![forbid(unsafe_code)]`).
Acesso ao Windows só via `afina-platform`.

## Arquivos

| Caminho | Responsabilidade |
|---|---|
| `tweak.rs` | Modelo puro: `Tweak`, `Category`, `Risk`, `Kind`, `RegOp` (`RegOp::new(..)`, `.missing_means_on()`), `Val`, `Post` |
| `catalog/{games,system,visual}.rs` | DADOS: um `static TWEAKS: &[Tweak]` por categoria. `catalog/mod.rs`: `all`, `by_category`, `recommended`, `find`, `count` + teste `catalog_is_consistent` |
| `engine/mod.rs` | Contrato: `is_on`, `set(t, on, store)`, `restore_original`; dispara `Post` (notificação ao Windows) |
| `engine/registry_ops.rs` | `Kind::Registry` (com rollback se falhar no meio) |
| `engine/dx_flag.rs` | `Kind::DxGlobalFlag`: mexe só na NOSSA chave dentro de `DirectXUserGlobalSettings` (`get_kv`/`set_kv`) |
| `engine/power_plan.rs` | `Kind::PowerPlan`: plano "Afina - Desempenho" via `powercfg`; `find_guid` |
| `backup.rs` | `BackupStore` (JSON, gravação atômica, arquivo corrompido vira `.json.corrompido`) |
| `startup.rs` | Programas de inicialização via `StartupApproved` (byte 0 par = ligado; ímpar = desligado) |
| `maintenance/` | `Tool` + um arquivo por ferramenta: `restore_point`, `temp`, `clock`, `dns`. Bloqueantes: rodar em thread |
| `error.rs` | `Error { Io, Command, Unsupported }` com `Display` em pt-BR; `is_access_denied()` |
| `paths.rs` | `%LOCALAPPDATA%\Afina`: `data_dir`, `backup_file`, `log_file` |

## Adicionar um ajuste (caso mais comum)

1. Escolha o arquivo da categoria em `catalog/` e acrescente um `Tweak` (ordem do arquivo = ordem na tela).
2. `id`: snake_case, único, **permanente** (fica gravado no backup dos usuários).
3. Para cada valor: `RegOp::new(hive, caminho, nome, on, off)`.
   - `on` = valor com o ajuste ligado; `off` = valor que desliga quando não há backup (normalmente o padrão do Windows).
   - Se a AUSÊNCIA do valor já significa "ligado" no Windows, use `.missing_means_on()`.
   - Escreve em HKLM ⇒ `admin: true`. Só vale após reiniciar ⇒ `reboot: true`.
4. `summary` = uma frase com o ganho; `details` = custo, quando usar, onde fica a mesma opção no Windows. Risco ≠ `Seguro` exige `details`.
5. `recommended: true` só se `Seguro`, sem admin e fora de `Visual` (o teste garante).
6. Precisa avisar o Windows para valer na hora? `post: Post::Mouse | Post::Theme`.
7. Inclua a fonte/evidência na descrição do PR e atualize a tabela do `README.md`.

## Regras do motor (não quebre)

- `is_on` sempre LÊ o Windows; nada de cache aqui (o cache é do app e é recalculado após cada ação).
- O primeiro snapshot em `BackupStore` vence: `turn_on` só insere se `!store.has(id)`.
- `turn_off`: restaura originais e remove o backup; se continuar "ligado" (padrão do Windows já era ligado), escreve os valores `off`.
- Falhou no meio de um `turn_on`? Desfaça o que já foi escrito e remova o backup recém-criado.
- Quem persiste o store é o chamador (app), após cada `set`/`restore_original`.
- Saída de `powercfg`/`w32tm` é localizada: parse só de GUID/números; aceite vírgula decimal (`parse_stripchart`).

## Testes

- Puros: no próprio arquivo (`#[cfg(test)]`). Integração com registro real: `tests/engine_registry.rs`, macro `test_tweak!` com chave própria sob `HKCU\Software\AfinaTest`.
- Proibido em teste: `powercfg /setactive`, `w32tm /resync`, ponto de restauração, qualquer HKLM ou chave fora de `AfinaTest`.
