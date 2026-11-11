# Arquitetura do Afina

Este documento é um mapa: diz onde cada coisa mora e quais regras a estrutura
protege. Detalhes de implementação estão no código.

## Visão geral

O Afina mostra uma lista de ajustes do Windows úteis para jogos. Para cada um:

1. **lê** o estado real no Windows;
2. ao ligar, **salva os valores originais** e escreve os novos;
3. ao desligar, **restaura os originais**.

Além disso, gerencia programas de inicialização e oferece ferramentas pontuais
(ponto de restauração, limpeza de temporários, relógio, DNS).

## Mapa do código

```
Cargo.toml              workspace virtual: versões de dependências, lints, perfil de release
crates/
  afina-platform/       Windows cru (registro, processos, UAC, informações do sistema)
  afina-core/           domínio (modelo, catálogo, motor, backup, inicialização, manutenção)
  afina-app/            interface egui (binário `afina`)
docs/                   este documento
.claude/rules/          instruções por módulo para o Claude Code
```

Direção das dependências — sempre para baixo:

```
afina-app ──► afina-core ──► afina-platform ──► windows-sys / winreg
    └──────────────────────────►┘ (janela, UAC, abrir URIs)
```

### `afina-platform`

Funções pequenas e seguras sobre o Win32. Valores do registro trafegam como
`RawValue` (tipo + bytes) para que qualquer valor possa ser restaurado byte a byte.
Comandos externos (`powercfg`, `w32tm`, ...) rodam sem janela via `process::run_hidden`.

### `afina-core`

- `tweak` — o modelo: um `Tweak` é **dado puro** (id, textos, risco, `Kind`).
- `catalog` — a lista de ajustes, um arquivo por categoria.
- `engine` — o comportamento, um módulo por `Kind`:
  `registry_ops` (valores de registro), `dx_flag` (uma chave dentro de
  `DirectXUserGlobalSettings`), `power_plan` (plano de energia próprio).
- `backup` — `BackupStore`, persistido em `%LOCALAPPDATA%\Afina\backup.json`.
- `startup` — programas de inicialização pelo mecanismo `StartupApproved` do Gerenciador de Tarefas.
- `maintenance` — `Tool`: ações pontuais e bloqueantes.

### `afina-app`

Fluxo de mão única, no estilo Elm:

```
views (pages/, shell/, dialogs)  ──Action──►  controller  ──►  afina-core
        ▲                                         │
        └────────────── AppState ◄────────────────┘
```

Views só leem `&AppState` e emitem `Action`. O `controller` é o único que muda
estado. Trabalho lento roda em threads (`jobs`), e a UI apenas consulta o resultado a cada frame.

## Invariantes

- **Estado nunca é presumido.** `engine::is_on` sempre consulta o Windows.
- **O primeiro backup vence.** Um ajuste aplicado duas vezes mantém os valores de antes da primeira vez.
- **Nada fica rodando.** Não há serviço, tarefa agendada nem processo após fechar o app.
- **`unsafe` só em `afina-platform`.** Os outros crates usam `#![forbid(unsafe_code)]`.
- **O core não conhece a UI** e a UI não escreve no registro.
- **Ids de ajuste são permanentes**, pois estão gravados nos backups dos usuários.
- **Testes nunca tocam configuração real**, só `HKCU\Software\AfinaTest`, protegido por `delete_test_tree`.

## Preocupações transversais

- **Idioma:** a interface e as mensagens de erro estão em português do Brasil; o código e os comentários, em inglês.
- **Erros:** `afina_core::Error` já traz a mensagem pronta para o usuário. Acesso negado sugere abrir como administrador.
- **Privilégios:** o app abre sem admin. Itens que exigem admin aparecem bloqueados até reabrir elevado (UAC).
- **Localização do Windows:** saídas de `powercfg`/`w32tm` vêm traduzidas. O código só extrai GUIDs e números.
- **Desempenho:** o app só redesenha com entrada do usuário ou com job ativo. Parado, fica em ~0% de CPU.
