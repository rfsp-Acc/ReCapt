# Ultima Sessao - ReCapt

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
