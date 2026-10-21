# ReCapt — Decisoes Tecnicas Fechadas

> Documento de referencia. Todas as decisoes abaixo foram discutidas e aprovadas.
> Atualizado em: 2026-09-21

---

## Framework

| Decisao | Escolha | Motivo |
|---|---|---|
| Desktop framework | **Tauri 2** | Bundle leve (~10 MB), Rust nativo, distribuivel mac + Windows |
| Frontend | **React + TypeScript** | Ecossistema maduro, compativel com Tauri |
| Backend | **Rust** | Acesso direto a APIs de audio nativas, performance |

---

## Captura

| Decisao | Escolha | Motivo |
|---|---|---|
| Audio (microfone) | `cpal` | Cross-platform, suporte CoreAudio (mac) e WASAPI (win) |
| Audio (sistema) | ScreenCaptureKit (mac) / WASAPI loopback (win) | Nativo, sem driver adicional |
| Gravacao de tela | FFmpeg subprocess | Simples, robusto, mesma abordagem nos dois sistemas |
| Controle de tela | Toggle pelo usuario em tempo real | Nao obrigatorio |

---

## Transcricao — Whisper local

| Hardware detectado | Modelo padrao | Tamanho |
|---|---|---|
| Windows CPU | `small` | 465 MB |
| Apple Silicon / GPU | `large-v3-turbo` | 1.6 GB |
| Hardware potente | `large-v3` | 3.0 GB |

- Modelo detectado automaticamente na primeira execucao
- Usuario pode trocar nas configuracoes
- Download unico, armazenado localmente
- Binding: `whisper-rs` (Rust)

---

## Analise de IA — 100% local, sem API

Framework: **`candle`** (HuggingFace, Rust puro — sem dependencia C++, compila cross-platform)

| RAM disponivel | Modelo padrao | Tamanho |
|---|---|---|
| 6 GB | Gemma 3 4B | ~2.5 GB |
| 8 GB | Qwen2.5 7B | ~4.7 GB |
| 16 GB+ | Gemma 3 12B | ~7.0 GB |

- Modelo detectado automaticamente pelo hardware disponivel
- Usuario pode trocar nas configuracoes
- Download na primeira execucao, uso offline permanente
- Suporte a reunioes em ingles e portugues
- Saida configuravel: portugues ou ingles

### O que a IA gera
- Organizacao e identificacao de temas
- Priorizacao por relevancia
- Mapeamento de atividades e proximos passos
- Sugestoes de abordagem por tema
- Nome descritivo para a sessao

---

## Custo para o usuario

**Zero.** Sem API key, sem conta, sem internet apos o primeiro download dos modelos.

---

## Saida — arquivos locais

```
~/ReCapt/
  2026-09/
    2026-09-21_nome-gerado-pela-ia/
      audio.wav
      gravacao.mp4          (se tela gravada)
      transcricao.md
      analise.md            (temas, priorizacao, sugestoes)
      acoes.md              (proximos passos)
```

- Nome temporario gerado por data/hora durante gravacao
- IA sugere nome descritivo apos analise
- Usuario confirma ou edita o nome
- Agrupado por mes

---

## Nomenclatura de sessoes

Fluxo hibrido:
1. Gravacao inicia imediatamente — sem pedir nome
2. Nome temporario: `2026-09-21_14h30`
3. Apos analise, IA sugere nome curto e descritivo
4. Usuario confirma com um clique ou edita
5. Pasta renomeada

---

## Configuracoes do usuario

- Modelo Whisper (detectado automaticamente, editavel)
- Modelo de IA (detectado automaticamente, editavel)
- Idioma da analise (portugues / ingles)
- Pasta de saida (padrao: `~/ReCapt/`)
- Toggle de gravacao de tela (padrao: desativado)
