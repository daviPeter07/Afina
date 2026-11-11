# Como contribuir

Obrigado pelo interesse! Antes de começar, leia `docs/ARCHITECTURE.md` (5 minutos).

## Propondo um ajuste novo

Abra uma issue com:

- o que muda no Windows (chave/valor ou comando);
- **evidência** do efeito: documentação oficial, benchmark reproduzível ou fonte técnica;
- o custo ou risco para o usuário.

Ajustes sem efeito comprovado não entram (veja "O que NÃO fazemos" no README).

## Implementando

1. Acrescente o `Tweak` no arquivo da categoria em `crates/afina-core/src/catalog/`.
2. Atualize a tabela do `README.md`.
3. Rode:

```powershell
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Regras rápidas

- `unsafe` só em `crates/afina-platform`, sempre com comentário `// SAFETY:`.
- Interface e mensagens em português do Brasil; código e comentários em inglês.
- Testes nunca alteram configurações reais. Use só chaves em `HKCU\Software\AfinaTest`.
- Não renomeie o `id` de um ajuste existente: ele está gravado nos backups dos usuários.
