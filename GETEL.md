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
| `flutter/lib/desktop/pages/desktop_setting_page.dart` | crédito ao RustDesk (AGPL-3.0) e link do código na tela Sobre |
| `res/icon.ico`, `res/tray-icon.ico`, `res/icon.png`, `res/32x32.png`, `res/64x64.png`, `res/128x128.png`, `res/128x128@2x.png`, `flutter/windows/runner/resources/app_icon.ico` | ícone da Getel no lugar do ícone do RustDesk |
| `flutter/lib/common.dart` | cores da marca Getel (`MyTheme.accent`, `accent50`, `accent80`, `button`, `idColor` = `0F62FE`) |
| `Cargo.toml` | metadados do exe (`[package.metadata.winres]`): ProductName, FileDescription e LegalCopyright da Getel |
| `flutter/windows/runner/Runner.rc` | metadados do exe do app Flutter: CompanyName, ProductName, FileDescription e LegalCopyright da Getel |
| `libs/portable/Cargo.toml` | metadados do exe empacotador (o `.exe` que o cliente abre): ProductName, FileDescription, OriginalFilename `getel-suporte.exe`, LegalCopyright e `description` |

## Arquivos novos da Getel

- `GETEL.md`
- `.github/workflows/getel-windows.yml`
- `src/getel.rs`
- `flutter/assets/logo.png`, `flutter/assets/logo_dark.png` (logo da tela do cliente, tema claro e escuro) e `flutter/assets/icon.png` (o `.gitignore` ignora `*png`, então são adicionados com `git add -f`)

## Como atualizar

1. Sync fork: sincronizar o fork com o upstream `rustdesk/rustdesk`.
2. Resolver conflitos, se houver.
3. Run workflow: rodar o workflow de build no GitHub Actions.
4. Roteiro de homologação: validar a versão gerada antes de liberar.
5. Release `1.5.1-getel.<n>`: publicar a release com o número sequencial `<n>`.

Os testes `getel::` rodam em uma chamada separada do `cargo test`, porque a configuração da Getel altera estáticos do processo e quebraria os testes originais se rodasse junto com eles.

## Versão de teste vs produção

O servidor e a Key ficam em `src/getel.rs`.
