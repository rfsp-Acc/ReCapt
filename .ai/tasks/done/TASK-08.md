# TASK-08: Versao instalavel (.app) e icone

**Status:** Concluido
**Agente:** Claude
**Data:** 2026-10-21

## Objetivo
Gerar o ReCapt.app para abrir pelo Dock ou Launchpad, sem terminal, com icone proprio e permissoes do macOS em nome do ReCapt.

## Criterios de Aceite
Automaticos:
- [x] Icone proprio gerado a partir de `src-tauri/icons/source/icon.svg`
- [x] `pnpm tauri build` gera `ReCapt.app` sem erros
- [x] App assinado ad-hoc, com hardened runtime e entitlement de microfone
- [x] `Info.plist` do bundle com as descricoes de permissao de microfone e audio do sistema
- [x] App instalado em `~/Applications` e indexado pelo Spotlight

Manuais (usuario):
- [x] Abrir o ReCapt pelo Launchpad ou Spotlight
- [x] macOS pede as permissoes em nome do ReCapt; gravacao e transcricao funcionam

## Notas
- Assinatura ad-hoc (`signingIdentity: "-"`) funciona so nesta maquina. Distribuir para outras pessoas exige Developer ID e notarizacao da Apple.
- Cada novo build ad-hoc muda a assinatura; o macOS pode pedir as permissoes de novo apos atualizar o app.
- `/Applications` sem permissao de escrita (Mac gerenciado); instalado em `~/Applications`.
- O `.dmg` falhou de forma intermitente ao desmontar a imagem temporaria ("Recurso ocupado"). O build padrao gera so o `.app`; para distribuir: `pnpm tauri build --bundles dmg`.
- Para atualizar o app instalado: `pnpm tauri build` e `ditto src-tauri/target/release/bundle/macos/ReCapt.app ~/Applications/ReCapt.app`.

## Validacao manual
2026-10-21: usuario abriu o app instalado, aceitou as permissoes em nome do ReCapt e gravou e transcreveu com sucesso.
