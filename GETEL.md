# GETEL.md

Registro do fork Getel de RustDesk, usado como app de suporte remoto da Getel.

## Versão base

RustDesk 1.5.1, commit 53996e3.

## Arquivos originais alterados

| Arquivo | Motivo |
| --- | --- |
| `src/common.rs` | aplica a configuração da Getel e separa `apply_custom_client_config` |
| `src/lib.rs` | declara `mod getel` |
| `src/platform/windows.rs` | desativa a licença pelo nome do exe (servidor e Key não podem ser trocados renomeando o arquivo) |

## Arquivos novos da Getel

- `GETEL.md`
- `.github/workflows/getel-windows.yml`
- `src/getel.rs`

## Como atualizar

1. Sync fork: sincronizar o fork com o upstream `rustdesk/rustdesk`.
2. Resolver conflitos, se houver.
3. Run workflow: rodar o workflow de build no GitHub Actions.
4. Roteiro de homologação: validar a versão gerada antes de liberar.
5. Release `1.5.1-getel.<n>`: publicar a release com o número sequencial `<n>`.

Os testes `getel::` rodam em uma chamada separada do `cargo test`, porque a configuração da Getel altera estáticos do processo e quebraria os testes originais se rodasse junto com eles.

## Versão de teste vs produção

O servidor e a Key ficam em `src/getel.rs`.
