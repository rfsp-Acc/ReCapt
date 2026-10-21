# TASK-03: Integracao do Whisper local

**Status:** Concluido
**Agente:** Claude
**Data:** 2026-10-21

## Objetivo
Transcrever o `audio.wav` da sessao com o Whisper rodando localmente e gerar o `transcricao.md`.

## Criterios de Aceite
Automaticos:
- [x] `whisper-rs` 0.16 com Metal no macOS e CPU nas demais plataformas
- [x] Modelo procurado em `<app_data>/models/whisper/` na ordem large-v3-turbo, large-v3, small
- [x] Command `transcribe(id)` roda fora da thread principal e emite `transcription-progress`
- [x] Transcricao automatica ao parar a gravacao; botao "Transcrever" para sessoes sem transcricao
- [x] `transcricao.md` com titulo em DD-MM-AAAA, duracao e falas com timestamp `[HH:MM:SS]`
- [x] Mensagens de erro claras: sem audio, sem modelo, sessao ja em transcricao
- [x] Testes unitarios passando; transcricao real de uma gravacao de 6,5 s concluida em 8,1 s (release, incluindo carga do modelo)

Manuais (usuario):
- [x] Gravar uma fala curta, parar e ver a barra de progresso ate o fim
- [x] Conferir o `transcricao.md` gerado na pasta da sessao
- [x] Usar "Transcrever" numa sessao antiga

## Notas
- Modelo `ggml-large-v3-turbo.bin` (1,6 GB) baixado manualmente em `~/Library/Application Support/com.recapt.app/models/whisper/`. Download e escolha por hardware ficam para a task de primeira execucao.
- Idioma em deteccao automatica, pois o suporte a multiplos idiomas ainda esta em aberto.
- O modelo e carregado a cada transcricao (~1 a 2 s); manter em memoria e otimizacao futura.
- Teste real: `RECAPT_MODEL=... RECAPT_AUDIO=... cargo test --release real_transcription -- --ignored --nocapture`.

## Validacao manual
2026-10-21: usuario confirmou transcricao automatica com progresso e transcricao.md gerado.
