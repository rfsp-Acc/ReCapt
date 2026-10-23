# Ultima Sessao - ReCapt

## Sessao 6 - Versao Windows, modelo na primeira execucao e atualizacao automatica | 2026-10-23

| Entrega | Resultado |
|---------|-----------|
| Windows | Instalador NSIS em pt-BR gerado pelo GitHub Actions; caminhos com `\` e arquivos do Explorer tratados; runtime do Visual C++ embutido |
| Modelo do Whisper | Cartao "Baixe o modelo de transcricao" com progresso: `small` no Windows, `large-v3-turbo` no Apple Silicon |
| TLS nativo | reqwest com native-tls; aws-lc quebrava a linkagem do Metal no macOS |
| Whisper portavel | `GGML_NATIVE=OFF` no Windows: sem AVX-512 do runner, fica AVX2 |
| Atualizacao automatica | Plugin updater; busca ao abrir e a cada 6 h, instala so com o app ocioso; versao nas Configuracoes |
| Release | `pnpm release X.Y.Z` cria tag; `release.yml` publica Windows e Mac em `rfsp-Acc/ReCapt-releases` |
| GPU no Windows | Whisper com Vulkan; build com `CARGO_TARGET_DIR=C:\t` (limite de 260 caracteres); instalador de teste gerado |
| Testado no Windows | Instalacao, gravacao e transcricao funcionando (lenta na CPU); commits ate `aada158` |

Arquivos: `.github/workflows/windows.yml`, `.github/workflows/release.yml`, `scripts/release.mjs`, `src-tauri/.cargo/config.toml`, `src-tauri/Cargo.toml`, `src-tauri/tauri.conf.json`, `src-tauri/capabilities/desktop.json`, `src-tauri/src/commands/model.rs`, `src-tauri/src/whisper/models.rs`, `src-tauri/src/error.rs`, `src-tauri/src/lib.rs`, `src-tauri/src/session/mod.rs`, `src/hooks/useWhisperModel.ts`, `src/hooks/useUpdater.ts`, `src/components/ModelCard.tsx`, `src/components/SettingsPanel.tsx`, `src/pages/Home.tsx`, `src/lib/format.ts`, `src/lib/tauri.ts`, `src/App.css`, `README.md`
Decisoes: Versoes so saem por comando (`pnpm release`), nao a cada push. Instaladores em repositorio publico separado (`ReCapt-releases`); codigo continua privado. Atualizacoes assinadas com `~/.tauri/recapt-updater.key` (build local exige `TAURI_SIGNING_PRIVATE_KEY`). Vulkan em vez de CUDA no Windows. Sem assinatura de codigo da Microsoft por enquanto (aviso do SmartScreen).
Proximos passos: 1) Testar o instalador com Vulkan na GTX 1650 (run 40016102594) 2) Usuario criar `rfsp-Acc/ReCapt-releases` e os segredos `TAURI_SIGNING_PRIVATE_KEY` e `RELEASES_TOKEN`; depois `pnpm release 0.2.0` 3) Instalar a 0.2.0 manualmente uma vez no Windows e no Mac (app do Mac nao foi atualizado: estava aberto)
Task atual: GPU no Windows - aguardando teste do usuario
