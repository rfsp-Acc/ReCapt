# TASK-11: Pasta própria por frente e gestão de reuniões

**Status:** Concluido
**Agente:** Claude
**Data:** 2026-10-22

## Objetivo
Permitir que cada frente grave numa pasta escolhida pelo usuário (ex.: `.ai/docs/reunioes` do repositório do projeto), valendo para todas as próximas reuniões da frente.

## Critérios de Aceite
Automáticos:
- [x] Caminho salvo em `<raiz>/<Projeto>/<Frente>/.recapt-pasta`; reuniões em `<pasta da frente>/<Tema>/AAAA-MM-DD_HHhMM Assunto/`
- [x] Id da sessão continua `<Projeto>/<Frente>/<Tema>/<pasta>`; listar, abrir, transcrever e recuperar funcionam na pasta da frente
- [x] Temas da pasta da frente aparecem como sugestão e na barra lateral; sessões antigas na raiz continuam listadas
- [x] "Usar padrão" volta a gravar na raiz; troca bloqueada durante a gravação
- [x] Cartão de gravação mostra onde a reunião será salva
- [x] Testes unitários passando; `cargo clippy` e `pnpm build` sem erros

Manuais (usuário, em modo dev):
- [x] Escolher `.ai/docs/reunioes` para a frente do ReCapt, gravar e conferir a pasta
- [x] Gravar de novo na mesma frente e ver que a pasta foi lembrada
- [x] "Usar padrão" e conferir que a próxima gravação volta para `~/ReCapt`

Também entregues nesta task:
- [x] Campos de texto e listas com a mesma altura e fonte em todo o app
- [x] Título da reunião sem assunto: Tema, senão Frente, senão Projeto

## Notas
- Sessões gravadas antes na raiz não são movidas.
- Áudio (`.wav`, `.mp3`, `.mp4`) em `.ai/docs/reunioes` fica fora do Git pelo `.gitignore`.
- Mover sessão pelo Finder: pasta de um tema define projeto, frente e tema; pasta fora da estrutura mantém o lugar no app via atalho `.<pasta>.recapt-local` na pasta do tema; cabeçalho do `transcricao.md` atualizado; tema vazio removido
- Renomear assunto: a pasta é renomeada onde estiver
- Excluir sessões: confirmação, pasta apagada por completo (sem Lixeira); tema vazio removido
- Renomear completa reuniões antigas: pede projeto, frente ou tema que faltam e move para a pasta do tema

## Validação manual
2026-10-22: usuário testou em modo dev e aprovou.
