# ReCapt — Estrutura Tecnica (Tauri)

## Stack

| Camada | Tecnologia |
|---|---|
| Interface | React + TypeScript |
| Backend | Rust (Tauri 2 commands) |
| Captura de audio | `cpal` + ScreenCaptureKit (mac) / WASAPI (win) |
| Captura de tela | FFmpeg subprocess |
| Transcricao | `whisper-rs` (bindings para whisper.cpp) |
| Analise de IA | `candle` (HuggingFace, Rust puro) |
| Arquivos | Sistema de arquivos local via Tauri FS API |

---

## Estrutura de pastas do projeto

```
recapt/
  src-tauri/                  <- backend Rust
    src/
      main.rs
      commands/
        capture.rs            <- iniciar/parar gravacao
        transcribe.rs         <- chamar whisper
        analyze.rs            <- chamar modelo IA local
        files.rs              <- gerenciar pastas e .md
      audio/
        recorder.rs           <- captura mic + sistema
        mixer.rs              <- mistura os dois canais
      screen/
        recorder.rs           <- captura de tela opcional (FFmpeg)
      whisper/
        engine.rs             <- wrapper do whisper-rs
        models.rs             <- gerenciar download de modelos
      ai/
        engine.rs             <- wrapper do candle
        models.rs             <- gerenciar download de modelos
        prompts.rs            <- prompts de analise
    Cargo.toml
    tauri.conf.json

  src/                        <- frontend React
    components/
      RecordButton.tsx
      ScreenToggle.tsx
      SessionList.tsx
      SessionDetail.tsx
      AnalysisPanel.tsx
      ModelSetup.tsx          <- configuracao na primeira execucao
    pages/
      Home.tsx
      Session.tsx
      Settings.tsx
    hooks/
      useRecording.ts
      useSession.ts
    lib/
      tauri.ts
    App.tsx

  models/                     <- modelos baixados (gitignore)
    whisper/
      large-v3-turbo.bin
    ai/
      qwen2.5-7b-q4.gguf

  output/                     <- saida das sessoes (gitignore)
    2026-09/
      2026-09-21_nome-sessao/
        audio.wav
        gravacao.mp4
        transcricao.md
        analise.md
        acoes.md
```

---

## Crates Rust principais

```toml
[dependencies]
tauri = { version = "2", features = ["shell-open"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["full"] }
cpal = "0.15"
hound = "3.5"
whisper-rs = "0.11"
candle-core = "0.8"
candle-nn = "0.8"
candle-transformers = "0.8"
chrono = "0.4"
anyhow = "1"
```

---

## Tauri Commands (API interna frontend -> backend)

```rust
#[tauri::command] start_recording()           -> Ok / Err
#[tauri::command] stop_recording()            -> session_id
#[tauri::command] toggle_screen_capture()     -> bool
#[tauri::command] transcribe(session_id)      -> stream de progresso
#[tauri::command] analyze(session_id)         -> stream de progresso
#[tauri::command] rename_session(id, name)    -> Ok
#[tauri::command] list_sessions()             -> Vec<Session>
#[tauri::command] get_session(id)             -> Session
#[tauri::command] open_session_folder(id)     -> Ok
#[tauri::command] get_system_info()           -> HardwareInfo
#[tauri::command] download_model(model_id)    -> stream de progresso
```

---

## Fluxo de dados

```
[mic]  ─┐
         ├─> mixer -> WAV -> audio.wav
[sys]  ─┘
[tela] ──────────────────> gravacao.mp4 (opcional)

         [usuario para gravacao]
                  |
         whisper-rs processa WAV
                  |
         transcricao.md gerado
                  |
         candle + modelo local recebe transcricao
                  |
         analise.md + acoes.md gerados
                  |
         IA sugere nome -> usuario confirma
                  |
         pasta renomeada
```

---

## Captura de audio por plataforma

### macOS
- Microfone: `cpal` com CoreAudio
- Sistema: ScreenCaptureKit (macOS 12.3+) — requer permissao do usuario

### Windows
- Microfone: `cpal` com WASAPI
- Sistema: WASAPI loopback — nativo, sem permissao adicional

---

## Modelos Whisper

| Modelo | Tamanho | Hardware alvo |
|---|---|---|
| `small` | 465 MB | Windows CPU |
| `large-v3-turbo` | 1.6 GB | Apple Silicon / GPU |
| `large-v3` | 3.0 GB | Hardware potente |

Detectado automaticamente. Armazenado em pasta de dados do app.

---

## Modelos de IA (candle)

| RAM | Modelo | Tamanho |
|---|---|---|
| 6 GB | Gemma 3 4B | ~2.5 GB |
| 8 GB | Qwen2.5 7B | ~4.7 GB |
| 16 GB+ | Gemma 3 12B | ~7.0 GB |

Detectado automaticamente. Usuario pode trocar nas configuracoes.

---

## Primeira execucao

1. App detecta hardware (RAM, CPU/GPU, Apple Silicon)
2. Sugere modelos ideais para Whisper e IA
3. Usuario confirma ou ajusta
4. Download dos modelos com barra de progresso
5. App pronto para uso
