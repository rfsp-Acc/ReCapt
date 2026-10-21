# TASK-06: Idioma da transcricao

**Status:** Concluido
**Agente:** Claude
**Data:** 2026-10-21

## Objetivo
Permitir escolher o idioma da transcricao e registrar no arquivo o idioma usado, para reunioes em ingles e reunioes que comecam num idioma e seguem em outro.

## Criterios de Aceite
Automaticos:
- [x] Opcao de idioma persistida nas configuracoes: Automatico, Portugues ou Ingles
- [x] A transcricao usa o idioma escolhido; em Automatico, registra o idioma detectado
- [x] `transcricao.md` traz no cabecalho o idioma e se foi detectado ou escolhido
- [x] "Transcrever novamente" em sessoes que ja tem transcricao, para corrigir o idioma
- [x] Testes unitarios passando; `cargo clippy` e `pnpm build` sem erros

Manuais (usuario):
- [x] Reuniao ou video em ingles transcrito corretamente em Automatico e em Ingles
- [x] Cabecalho do `transcricao.md` mostra o idioma certo

## Notas
- Em Automatico o whisper.cpp detecta o idioma nos primeiros 30 s e o mantem ate o fim; reunioes mistas devem usar o idioma predominante.

## Validacao manual
2026-10-21: usuario confirmou transcricao em ingles e cabecalho de idioma.
