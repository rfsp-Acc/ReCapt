# ReCapt — Nomenclatura de Sessoes

## O problema

Nomear antes de gravar cria atrito — voce precisa parar para pensar num nome antes de comecar. Nomear depois e mais natural, mas requer uma acao extra.

---

## Opcoes

### Opcao A — Nome automatico por data e hora
```
2026-09-21_14h30/
```
- Zero atrito para comecar
- Nome nao diz nada sobre o conteudo
- Util como fallback sempre

### Opcao B — Usuario nomeia antes de comecar
```
[ Nome da sessao: _________ ]  [ Iniciar ]
```
- Mais organizado, mas cria atrito
- Ruim para gravacoes espontaneas
- Util quando a reuniao e planejada

### Opcao C — Claude nomeia apos a analise (recomendado como padrao)
```
2026-09-21_14h30/           <- nome temporario durante gravacao
  -> renomeado para:
alinhamento-produto-xpto/   <- sugerido pelo Claude com base no conteudo
```
- Zero atrito para comecar (grava imediatamente)
- Nome significativo gerado automaticamente
- Usuario confirma ou edita o nome sugerido

### Opcao D — Hibrido (melhor experiencia)
1. Gravar comeca imediatamente com nome temporario `data_hora`
2. Apos analise, Claude sugere um nome curto e descritivo
3. Interface mostra: **"Sugestao: alinhamento-produto-xpto"** com opcao de editar
4. Usuario confirma com um clique ou digita outro nome
5. Pasta e renomeada

---

## Estrutura de pasta recomendada

```
~/ReCapt/
  2026-09/
    2026-09-21_alinhamento-produto-xpto/
      audio.mp3
      gravacao.mp4         (se tela gravada)
      transcricao.md
      analise.md
      acoes.md
    2026-09-19_retrospectiva-sprint/
      ...
```

Agrupa por mes para nao acumular tudo numa pasta so.

---

## Decisao sugerida

**Opcao D (Hibrido):** comecar sem fricao, Claude nomeia com base no conteudo, usuario confirma. E a experiencia mais fluida e que melhor aproveita a analise ja feita.
