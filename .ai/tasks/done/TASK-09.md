# TASK-09: Organizacao por Projeto / Frente / Data

**Status:** Concluido
**Agente:** Claude
**Data:** 2026-10-21

## Objetivo
Organizar as reunioes na hierarquia Projeto > Frente (ou tema) > Data, com criacao de projetos pelo app e escolha de projeto e frente antes de gravar.

## Criterios de Aceite
Automaticos:
- [x] Pastas em `<raiz>/<Projeto>/<Frente>/AAAA-MM-DD_HHhMM/`; projetos marcados com `.recapt-projeto`
- [x] "Adicionar projeto" abre janela para o nome; valida nome vazio, duplicado (sem diferenciar maiusculas) e caracteres proibidos
- [x] Gravacao exige projeto e frente; frente nova e criada na primeira gravacao, frente existente e reaproveitada mesmo com outra grafia
- [x] Sugestao das frentes existentes ao digitar; ultimo projeto e frente lembrados
- [x] Barra lateral com Todas as sessoes, arvore Projeto > Frente com contagem e "Sem projeto" para as sessoes antigas
- [x] Cabecalho do `transcricao.md` com projeto e frente; prompts de analise atualizados
- [x] Testes unitarios passando; `cargo clippy` e `pnpm build` sem erros

Manuais (usuario, em modo dev antes de atualizar o app instalado):
- [x] Criar projeto, gravar com uma frente nova e conferir a pasta criada
- [x] Gravar de novo na mesma frente e ver a sessao agrupada na barra lateral
- [x] Sessoes antigas aparecem em "Sem projeto"

## Notas
- Sessoes antigas (`AAAA-MM/`) nao sao migradas; continuam em "Sem projeto".
- Renomear ou excluir projetos e frentes ainda nao existe no app; pode ser feito no Finder (mantendo o arquivo `.recapt-projeto`).

## Validacao manual
2026-10-21: usuario testou em modo dev e aprovou.
