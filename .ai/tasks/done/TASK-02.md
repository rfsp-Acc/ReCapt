# TASK-02: Captura de audio (microfone + sistema)

**Status:** Concluido
**Agente:** Claude
**Data:** 2026-10-20

## Objetivo
Gravar o microfone e o audio do sistema ao mesmo tempo e gerar um unico `audio.wav` na pasta da sessao, com controles de iniciar, pausar, retomar e parar.

## Criterios de Aceite
Automaticos:
- [x] Microfone capturado via `cpal` (CoreAudio no macOS, WASAPI no Windows)
- [x] Audio do sistema capturado via loopback do `cpal` (process tap no macOS 14.6+, WASAPI loopback no Windows)
- [x] Se o loopback falhar, a gravacao continua so com o microfone e a interface mostra um aviso
- [x] Cada fonte e convertida para mono 16 kHz durante a gravacao, sem acumular o audio em memoria
- [x] Ao parar, as duas fontes sao mixadas em `audio.wav` (16 kHz, mono, 16 bits) e os temporarios sao removidos
- [x] Pausar e retomar descartam o audio do intervalo pausado e o cronometro ignora a pausa
- [x] Parar a gravacao nao trava a interface (mixagem fora da thread principal)
- [x] `Info.plist` com `NSMicrophoneUsageDescription` e `NSAudioCaptureUsageDescription`
- [x] Testes unitarios do resampler e do mixer passando; `cargo check`, `cargo test` e `pnpm build` sem erros

Manuais (usuario):
- [x] macOS pede permissao de microfone e de gravacao de audio do sistema na primeira gravacao
- [x] Gravacao de teste contem a voz do usuario e um som tocado no computador
- [x] Pausa de alguns segundos nao aparece no arquivo final

## Notas
- `audio.wav` em 16 kHz mono: formato que o Whisper consome direto (TASK-03) e ~115 MB por hora.
- O loopback no macOS usa o process tap do proprio `cpal` 0.18 (macOS 14.6+); o ScreenCaptureKit nao foi necessario.
- Em modo dev, o macOS pode atribuir a permissao ao terminal ou ao VS Code que iniciou o app.
- O toggle de tela continua como stub; a captura de tela sera uma task propria.

## Validacao manual
2026-10-21: usuario confirmou permissoes, gravacao com voz e audio do sistema e pausa descartada no arquivo final.
