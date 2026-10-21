# TASK-10: Tema como pasta e assunto da reuniao

**Status:** Concluido
**Agente:** Claude
**Data:** 2026-10-21

## Objetivo
Separar Frente e Tema em niveis proprios e permitir um assunto opcional por reuniao, para varias reunioes sobre o mesmo tema com assuntos diferentes.

## Criterios de Aceite
Automaticos:
- [x] Pastas em `<raiz>/<Projeto>/<Frente>/<Tema>/AAAA-MM-DD_HHhMM Assunto/`; assunto opcional
- [x] Campos Projeto, Frente, Tema (obrigatorios) e Assunto (opcional) no cartao de gravacao; frentes e temas sugeridos e reaproveitados
- [x] Projeto, frente e tema lembrados; assunto limpo apos cada gravacao
- [x] Barra lateral em tres niveis (Projeto > Frente > Tema), com "Sem tema" para sessoes gravadas direto na frente
- [x] Lista mostra horario, assunto (ou "sem assunto") e o caminho que nao esta no titulo da tela
- [x] Cabecalho do `transcricao.md` com assunto no titulo e Projeto, Frente e Tema; prompts atualizados
- [x] Testes unitarios passando; `cargo clippy` e `pnpm build` sem erros

Manuais (usuario, em modo dev):
- [x] Gravar com tema e assunto e conferir a pasta e o cabecalho da transcricao
- [x] Gravar de novo no mesmo tema com outro assunto e ver as duas reunioes agrupadas
- [x] Sessoes da TASK-09 (sem tema) aparecem em "Sem tema"

## Notas
- Sessoes gravadas na TASK-09, direto na frente, nao sao movidas.

## Validacao manual
2026-10-21: usuario testou em modo dev e aprovou.
