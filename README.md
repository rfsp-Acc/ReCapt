# ReCapt

Captura, transcrição e análise de reuniões no computador, 100% local.

## Desenvolvimento

Pré-requisitos: Rust (rustup), Node 20+, pnpm e FFmpeg.

```sh
pnpm install
pnpm tauri dev
```

As sessões gravadas ficam em `~/ReCapt/AAAA-MM/`. A documentação do projeto está em `.ai/`.

## Versões e atualização automática

Para publicar uma versão para Windows e Mac:

```sh
pnpm release 0.2.0
```

O script atualiza o número da versão, faz o commit, cria a tag `v0.2.0` e envia. A tag dispara o workflow `Release`, que publica os instaladores em [rfsp-Acc/ReCapt-releases](https://github.com/rfsp-Acc/ReCapt-releases/releases). Os apps instalados procuram versões novas ao abrir e a cada 6 horas; baixam em segundo plano e instalam quando não há gravação nem transcrição em andamento.

As atualizações são assinadas com a chave `~/.tauri/recapt-updater.key`. Sem ela não é possível publicar versões que os apps aceitem: guarde uma cópia em local seguro. Para gerar o app localmente:

```sh
TAURI_SIGNING_PRIVATE_KEY="$(cat ~/.tauri/recapt-updater.key)" TAURI_SIGNING_PRIVATE_KEY_PASSWORD="" pnpm tauri build
```

## Windows

Primeira instalação: baixe o `.exe` da última versão em [ReCapt-releases](https://github.com/rfsp-Acc/ReCapt-releases/releases/latest). Para um instalador de teste, sem publicar, use **Actions > Build Windows > Run workflow**.

O instalador traz tudo o que o app precisa, menos o modelo do Whisper. Na primeira abertura, o ReCapt oferece o download: `small` no Windows e `large-v3-turbo` no Mac com Apple Silicon. O modelo fica em `%APPDATA%\com.recapt.app\models\whisper\` no Windows e em `~/Library/Application Support/com.recapt.app/models/whisper/` no Mac.
