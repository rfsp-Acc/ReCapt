# Status - ReCapt
_Nenhuma sessao ainda._

---

## Sessao 1 - Planejamento e decisoes tecnicas | 2026-09-21

| Entrega | Resultado |
|---------|-----------|
| Estrutura `.ai/` | Criada com LAST-SESSION, STATUS, TASKS-INDEX, TASK-template |
| CLAUDE.md | Regras de escrita: sem travessao, UTF-8, gramatica pt-BR |
| `.claude/settings.json` | Permissoes amplas: Bash(*), Read/Edit/Write(*), Skill(*), MCPs Figma |
| Visao do produto | ReCapt: captura audio/tela, transcricao, analise IA, saida .md |
| Decisoes tecnicas fechadas | Tauri + React, cpal, FFmpeg, Whisper local, candle + Gemma/Qwen |
| Documentos de planejamento | 6 arquivos em `.ai/Plan/` cobrindo visao, arquitetura, comparativos e decisoes |

Decisoes: Tauri 2 + React. Whisper: small (Win CPU) / large-v3-turbo (Apple Silicon/GPU) / large-v3 (potente). IA local via candle: Gemma 3 4B (6GB RAM) / Qwen2.5 7B (8GB) / Gemma 3 12B (16GB+). Sem API key, 100% local e gratuito.

Proximos passos: 1) Iniciar estrutura do projeto Tauri 2) Implementar captura de audio 3) Integrar Whisper local

---

## Sessao 2 - Estrutura Tauri e captura de audio | 2026-10-21

| Entrega | Resultado |
|---------|-----------|
| Ambiente | Rust 1.99.1 instalado via rustup |
| TASK-01 | Projeto Tauri 2 + React + TS; commands de sessao; pastas em `~/ReCapt/AAAA-MM/AAAA-MM-DD_HHhMM` |
| Formato de data | Interface exibe DD-MM-AAAA; pastas mantem AAAA-MM-DD para ordenar |
| TASK-02 | Microfone + audio do sistema via `cpal` 0.18, mixados em `audio.wav` 16 kHz mono; pausa/retomada; validado manualmente |

Arquivos: `src-tauri/src/audio/`, `src-tauri/src/commands/`, `src-tauri/src/session/mod.rs`, `src-tauri/Info.plist`, `src/lib/format.ts`, `src/lib/tauri.ts`, `src/hooks/`, `src/components/`, `src/pages/Home.tsx`, `.ai/tasks/done/TASK-01.md`, `.ai/tasks/done/TASK-02.md`
Decisoes: Loopback do macOS pelo process tap do `cpal` (14.6+), sem ScreenCaptureKit. `audio.wav` em 16 kHz mono para alimentar o Whisper direto. Modelo Whisper da TASK-03 baixado manualmente; tela de primeira execucao sera task propria.
Proximos passos: 1) Instalar cmake e adicionar `whisper-rs` 2) Baixar `large-v3-turbo` e gerar `transcricao.md` 3) Registrar no indice as tasks restantes do plano (tela, IA, nome sugerido, primeira execucao)
Task atual: TASK-03 Integracao do Whisper local - Em andamento

Atualizacao 2026-10-21: TASK-03 (Whisper local) concluida e validada manualmente.

---

## Sessao 3 - Pasta, idioma, Bluetooth e app instalavel | 2026-10-21

| Entrega | Resultado |
|---------|-----------|
| TASK-04 | Escolha da pasta de salvamento persistida; padrao `~/ReCapt`; validado |
| Prompts | `prompts/` com analise, acoes e traducao em pt-BR para uso com IA externa |
| TASK-05 | Repositorio privado `rfsp-Acc/ReCapt`; commits ate `49801ad` |
| TASK-06 | Idioma da transcricao (auto/pt/en) no cabecalho; "Transcrever novamente"; validado |
| TASK-07 | Vigia de dispositivos, Bluetooth, escolha do microfone, recuperacao apos falha; 1 h transcrita em 150 s; validado |
| TASK-08 | ReCapt.app com icone proprio, assinado ad-hoc, instalado em `~/Applications`; validado |

Arquivos: `src-tauri/src/audio/`, `src-tauri/src/settings.rs`, `src-tauri/src/whisper/language.rs`, `src-tauri/src/commands/`, `src-tauri/tauri.conf.json`, `src-tauri/Entitlements.plist`, `src-tauri/icons/`, `src/components/`, `src/hooks/`, `src/pages/Home.tsx`, `prompts/`, `.ai/tasks/done/TASK-04..08.md`
Decisoes: Analise feita fora do app, com IA do usuario e os prompts de `prompts/`; IA local (candle) fora do escopo. Push por HTTPS com a conta `rfsp-Acc` (a `rsp-acc` e outra conta). App atualizado com `pnpm tauri build` + `ditto` para `~/Applications`.
Proximos passos: 1) Registrar em `decisoes-tecnicas.md` a analise externa 2) Decidir sobre identificacao de quem fala (Voce/Participantes, diarizacao) 3) Decidir gravacao de tela (implementar ou remover botao)
Task atual: Nenhuma em andamento - app pronto para uso pessoal
