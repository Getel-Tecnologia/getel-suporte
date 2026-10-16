# GETEL.md

Registro do fork Getel de RustDesk, usado como app de suporte remoto da Getel.

## Versão base

RustDesk 1.5.1, commit 53996e3.

## Arquivos originais alterados

| Arquivo | Motivo |
| --- | --- |

## Arquivos novos da Getel

- `GETEL.md`

## Como atualizar

1. Sync fork: sincronizar o fork com o upstream `rustdesk/rustdesk`.
2. Resolver conflitos, se houver.
3. Run workflow: rodar o workflow de build no GitHub Actions.
4. Roteiro de homologação: validar a versão gerada antes de liberar.
5. Release `1.5.1-getel.<n>`: publicar a release com o número sequencial `<n>`.

## Versão de teste vs produção

O servidor e a Key ficam em `src/getel.rs`.
