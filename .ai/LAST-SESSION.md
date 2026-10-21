# Ultima Sessao - ReCapt

## Sessao 2 - Estrutura Tauri e captura de audio | 2026-10-21

| Entrega | Resultado |
|---------|-----------|
| Ambiente | Rust 1.99.1 instalado via rustup |
| TASK-01 | Projeto Tauri 2 + React + TS; commands de sessao; pastas em `~/ReCapt/AAAA-MM/AAAA-MM-DD_HHhMM` |
| Formato de data | Interface exibe DD-MM-AAAA; pastas mantem AAAA-MM-DD para ordenar |
| TASK-02 | Microfone + audio do sistema via `cpal` 0.18, mixados em `audio.wav` 16 kHz mono; pausa/retomada; validado manualmente |
| TASK-03 | Whisper large-v3-turbo local (Metal); transcricao automatica com progresso; validado manualmente |

Arquivos: `src-tauri/src/audio/`, `src-tauri/src/commands/`, `src-tauri/src/session/mod.rs`, `src-tauri/Info.plist`, `src/lib/format.ts`, `src/lib/tauri.ts`, `src/hooks/`, `src/components/`, `src/pages/Home.tsx`, `.ai/tasks/done/TASK-01.md`, `.ai/tasks/done/TASK-02.md`
Decisoes: Loopback do macOS pelo process tap do `cpal` (14.6+), sem ScreenCaptureKit. `audio.wav` em 16 kHz mono para alimentar o Whisper direto. Modelo Whisper da TASK-03 baixado manualmente; tela de primeira execucao sera task propria.
Proximos passos: 1) Registrar no indice as tasks restantes do plano (tela, IA, nome sugerido, primeira execucao) 2) Implementar analise com IA local (candle) 3) Tela da sessao com transcricao
Task atual: TASK-03 Integracao do Whisper local - Concluido
