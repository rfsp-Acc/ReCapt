# Prompts de análise do ReCapt

Prompts para gerar, a partir do `transcricao.md` de uma sessão, os documentos de análise com qualquer IA. Foram escritos pensando no Claude, mas funcionam com outras IAs.

| Prompt | Gera | Quando usar |
|--------|------|-------------|
| [01-analise.md](01-analise.md) | `analise.md` | Sempre: temas, prioridades, decisões, riscos e sugestões de abordagem |
| [02-acoes.md](02-acoes.md) | `acoes.md` | Sempre: ações, responsáveis, prazos, próximos passos e pendências |
| [03-traducao.md](03-traducao.md) | `transcricao-pt.md` | Só quando a reunião não for em português |

A análise e as ações saem sempre em português, mesmo em reuniões em inglês. A tradução é opcional e independente.

## Ordem recomendada

1. `01-analise.md`
2. `02-acoes.md`, que reaproveita os nomes dos temas da análise
3. `03-traducao.md`, se precisar

## Com o Claude Code

Abra o Claude Code na pasta de saída do ReCapt (a pasta mostrada no app, em "Gravações e transcrições salvas em") e peça, por exemplo:

```text
Siga o prompt em /Users/rafael.pedroso/Developer/ReCapt/prompts/01-analise.md para a sessão 2026-10/2026-10-21_10h27 e salve o resultado como analise.md na pasta da sessão. Depois faça o mesmo com 02-acoes.md, gerando acoes.md.
```

Para várias reuniões de uma vez:

```text
Para cada sessão de 2026-10 que tenha transcricao.md e ainda não tenha analise.md, aplique os prompts 01-analise.md e 02-acoes.md da pasta /Users/rafael.pedroso/Developer/ReCapt/prompts e salve os resultados na pasta da sessão.
```

## Com o claude.ai

1. Crie um Projeto e cole o conteúdo do prompt desejado nas instruções do Projeto, ou cole o prompt no início da conversa.
2. Anexe o `transcricao.md` da sessão ou cole o conteúdo dentro das tags `<transcricao>`, no fim do prompt.
3. Salve a resposta como `analise.md`, `acoes.md` ou `transcricao-pt.md` na pasta da sessão.

## Dicas

- Use o modelo mais capaz disponível. Profundidade de análise depende muito do modelo.
- Reuniões longas (acima de 1 hora) cabem no contexto do Claude, mas a tradução pode precisar ser feita em partes.
- Se a análise vier rasa em algum tema, peça: "Aprofunde o tema N seguindo a mesma estrutura".
- Antes de enviar transcrições com informações de clientes para uma IA na nuvem, confirme se a ferramenta é aprovada pela política da sua empresa.
