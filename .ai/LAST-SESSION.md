# Ultima Sessao - ReCapt

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
