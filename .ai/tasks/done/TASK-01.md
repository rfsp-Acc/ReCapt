# TASK-01: Estrutura inicial do projeto Tauri 2 + React

**Status:** Concluido
**Agente:** Claude
**Data:** 2026-10-20

## Objetivo
Criar o esqueleto do app conforme `.ai/Plan/estrutura-tecnica-tauri.md`.

## Criterios de Aceite
- [x] Projeto Tauri 2 + React + TypeScript compilando (`cargo check` e `pnpm build`)
- [x] Commands de gravacao (stub), listagem e abertura de sessoes
- [x] Pastas de sessao em `~/ReCapt/AAAA-MM/AAAA-MM-DD_HHhMM`
- [x] Tela inicial com botao de gravacao, cronometro, toggle de tela e lista de sessoes
- [x] Datas exibidas na interface no formato DD-MM-AAAA

## Notas
- Rust instalado via rustup (1.99.1).
- A gravacao ainda nao captura audio: apenas gerencia estado e cria a pasta. A captura real fica na TASK-02.
- O nome das pastas mantem AAAA-MM-DD para ordenacao cronologica; a conversao para DD-MM-AAAA e feita em `src/lib/format.ts`.
