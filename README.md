# ReCapt

Captura, transcrição e análise de reuniões no computador, 100% local.

## Desenvolvimento

Pré-requisitos: Rust (rustup), Node 20+, pnpm e FFmpeg.

```sh
pnpm install
pnpm tauri dev
```

As sessões gravadas ficam em `~/ReCapt/AAAA-MM/`. A documentação do projeto está em `.ai/`.

## Windows

O instalador é gerado pelo GitHub Actions: em **Actions > Build Windows > Run workflow**, o arquivo `ReCapt_x.y.z_x64-setup.exe` fica disponível como artefato ao fim da execução.

O instalador traz tudo o que o app precisa, menos o modelo do Whisper. Na primeira abertura, o ReCapt oferece o download: `small` no Windows e `large-v3-turbo` no Mac com Apple Silicon. O modelo fica em `%APPDATA%\com.recapt.app\models\whisper\` no Windows e em `~/Library/Application Support/com.recapt.app/models/whisper/` no Mac.
