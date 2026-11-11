# Afina

**Windows afinado para jogos — leve, transparente e reversível.**

Afina é um app de código aberto, escrito em Rust, que ajusta o Windows 10/11 para rodar jogos melhor. É um único `.exe`, sem instalação, sem internet e sem telemetria.

> 🇺🇸 *Afina is a lightweight, open-source Windows tuner for gaming written in Rust. Every tweak is documented, shows the real current state and can be undone. UI in Brazilian Portuguese.*

## Por que outro otimizador?

A maioria dos "otimizadores" aplica dezenas de truques de registro sem efeito comprovado, não mostra o que mudou e não deixa voltar atrás. O Afina segue quatro regras:

1. **Só ajustes com efeito documentado.** Cada um explica o que faz, o ganho esperado e o custo.
2. **Estado real.** Cada botão lê o Windows e mostra o que está *de fato* configurado.
3. **Tudo reversível.** Antes da primeira mudança, os valores originais são salvos em `%LOCALAPPDATA%\Afina\backup.json`. A aba **Restaurar** desfaz tudo.
4. **Nada escondido.** Nenhum serviço residente, nenhuma tarefa agendada, nenhuma conexão de rede (exceto a checagem de horário com `time.windows.com`, feita pelo próprio `w32tm` do Windows).

## O que ele faz

| Área | Ajuste | Risco | Admin |
|---|---|---|---|
| Jogos | Modo de Jogo | Seguro | – |
| Jogos | Desligar gravação em segundo plano (Xbox Game Bar) | Seguro | – |
| Jogos | Otimizações para jogos em janela (DirectX) | Seguro | – |
| Jogos | Desligar aceleração do mouse | Seguro | – |
| Jogos | Agendamento de GPU acelerado por hardware (HAGS) | Moderado | ✔ |
| Jogos | Plano de energia Desempenho Máximo | Moderado | ✔ |
| Sistema | Bloquear apps da Store em segundo plano | Moderado | – |
| Sistema | Desligar Inicialização Rápida | Seguro | ✔ |
| Sistema | Relógio da BIOS em UTC (dual boot com Linux) | Moderado | ✔ |
| Sistema | Desligar Integridade de Memória (HVCI) | Avançado | ✔ |
| Visual | Desligar transparência / animações | Seguro | – |

Além disso:

- **Inicialização:** liga e desliga programas que abrem com o Windows, pelo mesmo mecanismo do Gerenciador de Tarefas (`StartupApproved`). Nada é desinstalado.
- **Ferramentas:** ponto de restauração, limpeza de temporários (só arquivos com mais de 24 h; nunca o cache de shaders), acerto do relógio e limpeza do cache de DNS.
- **Diagnóstico de relógio:** relógio errado faz jogos online recusarem o login (ex.: **VAL 7** no VALORANT). O Afina mostra isso na tela inicial e corrige com um clique.

## O que NÃO fazemos (de propósito)

Estes ajustes aparecem em muitos otimizadores, mas não têm benefício comprovado em jogos modernos ou trazem mais risco que ganho:

- **"Desligar o algoritmo de Nagle" / `TcpAckFrequency`:** jogos usam UDP; isso não afeta o ping.
- **`NetworkThrottlingIndex` / `SystemResponsiveness`:** efeito não mensurável em sistemas atuais.
- **Desativar serviços do Windows em massa:** quebra recursos (Wi-Fi, impressão, atualizações) sem ganho real de FPS.
- **"Limpar RAM" em segundo plano:** o Windows já gerencia a memória; esvaziar o cache deixa tudo mais lento.
- **Apagar o cache de shaders:** causa travadas até os shaders serem recompilados.
- **Desligar Windows Defender / atualizações:** risco de segurança grave.

Quer propor um ajuste? Abra uma issue com **evidência** (benchmark, documentação oficial ou fonte técnica).

## Download e uso

1. Baixe `afina.exe` na página de *Releases*.
2. Abra. Ajustes marcados com **Admin** pedem o botão **"Abrir como administrador"**.
3. Na tela inicial, clique em **Aplicar recomendados**.

## Compilar a partir do código

Requisitos: [Rust](https://rustup.rs) estável (1.88+).

```powershell
git clone <url-do-repositorio>
cd afina
cargo run -p afina-app      # versão de desenvolvimento
cargo build --release       # gera target\release\afina.exe
cargo test --workspace      # testes (usam apenas a chave HKCU\Software\AfinaTest)
```

Dica: `AFINA_PAGINA=jogos` (ou `sistema`, `visual`, `inicializacao`, `ferramentas`, `restaurar`, `sobre`) abre o app direto numa aba, útil para capturas de tela.

O toolchain **MSVC** (padrão do rustup) precisa do *Visual Studio Build Tools* com C++. Alternativa mais leve: o toolchain **GNU** (`rustup default stable-x86_64-pc-windows-gnu`) com o [WinLibs](https://winlibs.com) (MinGW) no `PATH`.

## Estrutura do código

Workspace Cargo com três crates. As dependências vão sempre para baixo (`app → core → platform`):

| Crate | Responsabilidade |
|---|---|
| `crates/afina-platform` | Windows cru: registro (valores brutos para backup fiel), processos sem janela, UAC, informações do sistema. Único lugar com `unsafe` |
| `crates/afina-core` | Domínio: modelo e catálogo de ajustes, motor que aplica/desfaz e lê o estado real, backup, inicialização, manutenção |
| `crates/afina-app` | Interface egui: páginas, componentes, tema; as views emitem ações e um controlador as executa |

Mapa completo e regras da arquitetura em [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md). Para contribuir, veja o [`CONTRIBUTING.md`](CONTRIBUTING.md).

### Adicionando um ajuste

Acrescente um `Tweak` no arquivo da categoria em `crates/afina-core/src/catalog/`, informando o valor **ligado** e o **desligado** de cada chave (e `.missing_means_on()` se a ausência do valor já equivale a "ligado"). O motor cuida do backup, da leitura do estado e da restauração, a interface lista o ajuste sozinha, e o teste `catalog_is_consistent` valida a entrada.

## Referências

- Microsoft — *Options to optimize gaming performance in Windows 11* (Integridade de Memória e Virtual Machine Platform). Cobertura: [Tom's Hardware](https://www.tomshardware.com/news/microsoft-windows-11-gaming-boost-tips), [TechSpot](https://www.techspot.com/news/96246-microsoft-releases-guide-optimize-windows-11-gaming-performance.html).
- [WinUtil](https://github.com/ChrisTitusTech/winutil) (Chris Titus Tech, MIT): referência dos valores padrão do Windows para Modo de Jogo, mouse e apps em segundo plano.
- Documentação do Windows: `powercfg`, `w32tm`, `RealTimeIsUniversal`, `StartupApproved`.

## Licença

[MIT](LICENSE). Use, modifique e distribua livremente.

> O Afina altera configurações do sistema. Embora tudo seja reversível, use por sua conta e risco e crie um ponto de restauração antes (o app oferece isso).
