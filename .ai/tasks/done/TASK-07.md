# TASK-07: Robustez da gravacao e dispositivos Bluetooth

**Status:** Concluido
**Agente:** Claude
**Data:** 2026-10-21

## Objetivo
Garantir que reunioes reais sejam gravadas por inteiro, mesmo com troca de dispositivo de audio, falha do app ou longa duracao.

## Criterios de Aceite
Automaticos:
- [x] Vigia de dispositivos: microfone e audio do sistema seguem o dispositivo padrao; streams reabertos ao trocar ou desconectar
- [x] Faixas realinhadas com silencio quando ficam mais de 0,5 s atras do relogio da gravacao (que desconta pausas)
- [x] Trocas e falhas de dispositivo aparecem como avisos com horario na interface
- [x] Cabecalho dos WAV temporarios atualizado a cada 5 s
- [x] Ao abrir o app, gravacoes interrompidas sao recuperadas em `audio.wav`
- [x] Qualquer formato PCM e qualquer taxa de amostragem (Bluetooth usa 8, 16 ou 24 kHz); taxa pode mudar ao trocar de dispositivo
- [x] Escolha do microfone nas configuracoes (padrao do sistema ou dispositivo especifico), valendo ate durante a gravacao
- [x] Teste de transcricao com 1 hora de audio: tempo e memoria medidos
- [x] Testes unitarios passando; `cargo clippy` e `pnpm build` sem erros

Manuais (usuario):
- [x] Conectar e desconectar o AirPods durante uma gravacao, falando e tocando som antes e depois
- [x] Escolher o microfone do Mac com fone Bluetooth conectado e confirmar audio do fone em boa qualidade
- [x] Fechar o app a forca durante uma gravacao e reabrir: a sessao aparece com audio

## Notas
- Problema encontrado: o tap de loopback do `cpal` fica preso ao dispositivo de saida; ao trocar a saida para outro dispositivo, a faixa do sistema gravava silencio sem erro.
- O `cpal` so segue o padrao automaticamente em streams de saida; os de entrada precisam ser reabertos pelo app.
- Teste de 1 h (release, large-v3-turbo, Metal): 150 s de transcricao, pico de 2,7 GB de memoria, cobertura ate 01:00:00 sem lacunas.
- Ao usar o microfone de fones Bluetooth (perfil de chamada), o macOS reduz a qualidade do audio do fone; o microfone do computador evita isso.
- O audio do sistema segue a saida padrao. Apps de reuniao configurados para enviar o som a outro dispositivo nao sao capturados.

## Validacao manual
2026-10-21: usuario confirmou troca de dispositivo Bluetooth durante a gravacao, escolha do microfone e recuperacao apos fechamento forcado. O teste revelou corrida na recuperacao (efeito duplo do React em modo dev), corrigida com trava e escrita atomica do audio.wav.
