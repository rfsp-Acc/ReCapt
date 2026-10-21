# TASK-04: Escolha da pasta de salvamento

**Status:** Concluido
**Agente:** Claude
**Data:** 2026-10-21

## Objetivo
Permitir que o usuario escolha a pasta onde as sessoes (audio e transcricao) sao salvas.

## Criterios de Aceite
- [x] Seletor de pastas nativo (plugin dialog, chamado pelo Rust)
- [x] Escolha persistida em `<app_config>/settings.json`; padrao `~/ReCapt`
- [x] Validacao de escrita no momento da escolha
- [x] Troca bloqueada durante gravacao
- [x] Lista mostra apenas pastas de mes `AAAA-MM`, mesmo em pastas com outros conteudos
- [x] Botoes "Alterar pasta", "Usar padrao" e "Abrir"
- [x] Testes unitarios de settings e listagem; validado manualmente pelo usuario em 2026-10-21

## Notas
- Sessoes antigas nao sao movidas ao trocar a pasta.
