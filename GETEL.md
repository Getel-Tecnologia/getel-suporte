# GETEL.md

Registro do fork do RustDesk usado como app de suporte remoto da Getel (Getel Suporte).

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
| `Cargo.toml` | metadados do exe (`[package.metadata.winres]`): ProductName, FileDescription e LegalCopyright da Getel, OriginalFilename `GetelSuporte.exe` |
| `flutter/windows/runner/Runner.rc` | metadados do exe do app Flutter: CompanyName, ProductName, FileDescription e LegalCopyright da Getel, InternalName `GetelSuporte`, OriginalFilename `GetelSuporte.exe` |
| `libs/portable/Cargo.toml` | metadados do exe empacotador (o `.exe` que o cliente abre): ProductName, FileDescription, CompanyName, OriginalFilename `GetelSuporte.exe`, LegalCopyright e `description` |

## Arquivos novos da Getel

- `GETEL.md`
- `.github/workflows/getel-windows.yml`
- `src/getel.rs`
- `flutter/assets/logo.png`, `flutter/assets/logo_dark.png` (logo da tela do cliente, tema claro e escuro) e `flutter/assets/icon.png` (o `.gitignore` ignora `*png`, então são adicionados com `git add -f`)

## Nome interno vs nome exibido

- **Nome interno: `GetelSuporte`** (sem espaço). Fica em `APP_NAME`, em `src/getel.rs`. O RustDesk usa esse nome para o serviço do Windows, as pastas, o registro, o atalho de link (URI) e o nome do exe instalado (`GetelSuporte.exe`).
- **Nome exibido: "Getel Suporte"** (com espaço). É o que o cliente vê nas propriedades do arquivo (ProductName e FileDescription) e na tela Sobre.
- Por que o nome interno não pode ter espaço: o RustDesk monta comandos do Windows com esse nome **sem aspas** (`sc create`, `sc start`, `sc stop`, `sc delete`, `taskkill /IM`, `reg add` da desinstalação e o registro do link). Com espaço, esses comandos quebram. O código do RustDesk também assume que o nome tem só letras, números e hífen.
- Por isso o exe de dentro do pacote se chama `GetelSuporte.exe`: o instalador renomeia o exe para `{nome interno}.exe` e o `taskkill` procura por esse nome. Ao abrir o pacote, os arquivos são extraídos em `%LOCALAPPDATA%\getelsuporte`, separado do RustDesk oficial.

## Como gerar uma versão (Run workflow)

O workflow se chama **"Getel Windows"**. Para rodar: no GitHub, abra **Actions → Getel Windows → Run workflow**, preencha `getel_rev` (o número da revisão, por exemplo `1`) e clique em **Run workflow**.

O arquivo gerado se chama `getel-suporte-<versão>-getel.<getel_rev>-x86_64.exe` e fica no artifact `getel-suporte-windows-x64`.

Os testes `getel::` rodam em uma chamada separada do `cargo test`, porque a configuração da Getel altera estáticos do processo e quebraria os testes originais se rodasse junto com eles.

## Como atualizar com o RustDesk original

1. **Sync fork:** na página do fork no GitHub, clique em **Sync fork**.
   - Se o GitHub oferecer **Discard N commits**, **NUNCA clique**. Esse botão apaga o trabalho da Getel.
   - Se o GitHub disser que há conflitos, siga o caminho local abaixo.
2. **Caminho local (quando há conflitos):**
   1. `git fetch upstream`
   2. `git merge upstream/master`
   3. Resolva os conflitos. Use a tabela "Arquivos originais alterados" para saber o que é da Getel em cada arquivo.
   4. Faça o commit do merge.
   5. Rode o workflow **"Getel Windows"** (veja acima) e confira que passou.
   6. Só então `git push`.
3. **Desative workflows novos do upstream:** depois do merge, abra **Actions** e veja se apareceu algum workflow novo do RustDesk. Pela linha de comando:
   - `gh workflow list --all` mostra todos;
   - `gh workflow disable "<nome do workflow>"` desativa um. Deixe ativo só o "Getel Windows".
4. **Deriva do workflow:** o `getel-windows.yml` é uma cópia de `flutter-build.yml` do RustDesk, no commit registrado na primeira linha do arquivo (hoje `53996e3`). A cada atualização:
   1. compare com `git diff <commit-registrado> upstream/master -- .github/workflows/flutter-build.yml`;
   2. traga para o `getel-windows.yml` as mudanças de `VERSION`, `FLUTTER_VERSION`, versão do Rust, `VCPKG_COMMIT_ID` e dos passos de Windows;
   3. atualize o commit registrado na primeira linha do `getel-windows.yml`.
5. **Roteiro de homologação:** valide a versão gerada antes de liberar.
6. **Release `<versão>-getel.<n>`:** publique a release com o número sequencial `<n>` (o mesmo `getel_rev` usado no workflow).

## Versão de teste vs produção

O servidor e a Key ficam em `src/getel.rs`, nas constantes `SERVER` e `KEY`. Os testes usam essas constantes, então trocar os valores não quebra o build.

Para trocar do servidor de teste para o de produção (nuvem):

1. Edite `SERVER` e `KEY` em `src/getel.rs`.
   - `SERVER`: o endereço do servidor da nuvem (`suporte.geteltecnologia.com.br`).
   - `KEY`: o conteúdo do arquivo `id_ed25519.pub` do servidor da nuvem (o novo, gerado lá).
2. Na Cloudflare, o DNS `suporte.geteltecnologia.com.br` fica com a **nuvem cinza** (sem proxy).
3. No firewall do VPS, deixe abertas as portas **21115 a 21117 TCP** e **21116 UDP**.
4. Rode o workflow "Getel Windows" e faça a homologação de novo.

## Convivência com o RustDesk oficial

O RustDesk oficial e o Getel Suporte usam o mesmo programa auxiliar do modo de privacidade, `RuntimeBroker_rustdesk.exe`. Por isso, instalar ou desinstalar o Getel Suporte pode encerrar esse auxiliar do RustDesk oficial, se os dois estiverem na mesma máquina. Fica só registrado; não há correção nesta versão.
