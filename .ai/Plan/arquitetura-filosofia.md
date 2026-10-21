# ReCapt — Filosofia de Arquitetura

## Contexto de decisao

- Plataformas: macOS + Windows
- Dados: 100% locais, saida em arquivos `.md` organizados por data
- Distribuicao futura: gera instalador nativo
- Analise: pos-gravacao
- Interface: propria (Tauri + React)
- Custo para o usuario: zero (sem API key, sem conta)

---

## Camadas do sistema

```
[ Captura ]  ->  [ Processamento ]  ->  [ Analise ]  ->  [ Saida ]
  audio            transcricao           IA local         .md files
  tela (opt)       Whisper local         candle           interface
  cpal + FFmpeg    whisper-rs            Gemma/Qwen
```

---

## Decisoes principais

### 1. Framework — Tauri 2 + React

- Bundle ~10 MB vs ~150 MB do Electron
- Backend Rust com acesso direto as APIs nativas de audio
- Frontend React (ecossistema maduro)
- Instalador nativo para macOS e Windows

### 2. Captura de audio

**macOS:** `cpal` para microfone + ScreenCaptureKit para audio do sistema (macOS 12.3+)
**Windows:** `cpal` com WASAPI loopback — funciona nativamente para ambos

Gravacao de tela via FFmpeg subprocess — mesma abordagem nos dois sistemas, toggle controlado pelo usuario.

### 3. Transcricao — Whisper local via `whisper-rs`

Modelo detectado automaticamente pelo hardware:

| Hardware | Modelo |
|---|---|
| Windows CPU | `small` (465 MB) |
| Apple Silicon / GPU | `large-v3-turbo` (1.6 GB) |
| Hardware potente | `large-v3` (3.0 GB) |

Suporte nativo a sotaques variados (indiano, brasileiro, americano) em ingles.

### 4. Analise — IA local via `candle` (HuggingFace, Rust puro)

Modelo detectado automaticamente pela RAM disponivel:

| RAM | Modelo |
|---|---|
| 6 GB | Gemma 3 4B (~2.5 GB) |
| 8 GB | Qwen2.5 7B (~4.7 GB) |
| 16 GB+ | Gemma 3 12B (~7.0 GB) |

Sem API key. Sem internet apos o primeiro download. Analise em portugues ou ingles.

### 5. Saida — arquivos `.md` estruturados

```
~/ReCapt/
  2026-09/
    2026-09-21_nome-gerado-pela-ia/
      audio.wav
      gravacao.mp4
      transcricao.md
      analise.md
      acoes.md
```

Nome da sessao sugerido pela IA, confirmado pelo usuario.

---

## Fluxo principal

1. Usuario abre o app e clica em **Iniciar**
2. App captura audio (microfone + sistema)
3. Usuario decide se ativa gravacao de tela (toggle)
4. Usuario clica em **Parar**
5. Whisper transcreve o audio localmente
6. IA analisa a transcricao e gera temas, acoes e sugestoes
7. IA sugere nome para a sessao — usuario confirma
8. Arquivos `.md` salvos na pasta organizada por data
