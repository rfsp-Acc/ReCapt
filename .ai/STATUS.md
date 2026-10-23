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

---

## Sessao 4 - Nova interface e hierarquia Projeto / Frente / Tema | 2026-10-21

| Entrega | Resultado |
|---------|-----------|
| Interface | Cartao de gravacao com medidores de nivel, sessoes por dia, painel de configuracoes, avisos, modo escuro; commit `476d5df` |
| TASK-09 | Projetos com janela "Novo projeto", barra lateral, gravacao exige projeto e frente; commit `8bc38eb` |
| TASK-10 | Tema como pasta e assunto opcional: `<Projeto>/<Frente>/<Tema>/AAAA-MM-DD_HHhMM Assunto`; commit `ab7e24b` |
| App oficial | Atualizado em `~/Applications` apos cada aprovacao em modo dev |

Arquivos: `src-tauri/src/session/mod.rs`, `src-tauri/src/commands/`, `src-tauri/src/audio/recorder.rs`, `src-tauri/src/whisper/transcript.rs`, `src/components/`, `src/hooks/`, `src/pages/Home.tsx`, `src/App.css`, `prompts/01-analise.md`, `prompts/02-acoes.md`, `.ai/tasks/done/TASK-09.md`, `.ai/tasks/done/TASK-10.md`
Decisoes: Mudancas sao testadas primeiro em `pnpm tauri dev`; app oficial e push so apos aprovacao do usuario. Projetos marcados com `.recapt-projeto`. Sessoes antigas (por mes ou direto na frente) nao sao migradas.
Proximos passos: 1) Renomear e excluir projetos, frentes e temas pelo app 2) Identificar quem fala (Voce / Participantes) 3) Registrar em `decisoes-tecnicas.md` a analise externa
Task atual: Nenhuma em andamento - TASK-10 concluida

---

## Sessao 5 - Pasta por frente e gestao de reunioes | 2026-10-22

| Entrega | Resultado |
|---------|-----------|
| TASK-11 | Pasta propria por frente (`.recapt-pasta`), ex.: `.ai/docs/reunioes`; cartao mostra destino; commit `ba9c451` |
| Mover | Pelo Finder: pasta de tema define o lugar; fora da estrutura mantem o lugar no app via `.<pasta>.recapt-local` |
| Renomear | Troca o assunto onde a pasta estiver; completa reunioes antigas sem projeto, frente ou tema |
| Excluir | Confirmacao e remocao total da pasta (sem Lixeira); tema vazio removido |
| Interface | Campos com mesma altura e fonte; titulo sem assunto usa Tema, Frente ou Projeto |
| App oficial | Atualizado em `~/Applications` apos aprovacao; push feito |

Arquivos: `src-tauri/src/session/mod.rs`, `src-tauri/src/commands/files.rs`, `src-tauri/src/whisper/transcript.rs`, `src/components/RenameSessionDialog.tsx`, `src/components/DeleteSessionDialog.tsx`, `src/components/CaptureCard.tsx`, `src/components/SessionList.tsx`, `src/pages/Home.tsx`, `src/App.css`, `.gitignore`, `.ai/tasks/done/TASK-11.md`
Decisoes: Id da sessao continua `<Projeto>/<Frente>/<Tema>/<pasta>` mesmo fora da raiz. Audio de `.ai/docs/reunioes` fica fora do Git. Excluir nao usa Lixeira.
Proximos passos: 1) Renomear e excluir projetos, frentes e temas pelo app 2) Identificar quem fala (Voce / Participantes) 3) Registrar em `decisoes-tecnicas.md` a analise externa
Task atual: Nenhuma em andamento - TASK-11 concluida

---

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
