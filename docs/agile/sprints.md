# Sprints

Sprints de 1 semana. Cada sprint entrega um **incremento utilizável e testado**
(ver [Definition of Done](definition-of-done.md)).

| Sprint | Meta | Histórias | Pontos |
|-------:|------|-----------|-------:|
| 0 | Fundação: workspace em camadas, CI, backlog inicial | — (habilitadores) | — |
| 1 | Domínio puro: pessoas, fonética, modelo de sotaque | US-01 (+ base de US-03/04) | 2 |
| 2 | Clonagem: amostras, aprendizado de sotaque e voz | US-02, US-03, US-04, US-10 | 24 |
| 3 | Mensagens e agendamentos ponta a ponta na CLI | US-05, US-06, US-07, US-08, US-09 | 14 |
| 4 | Tom de voz por mensagem e app Android com CI/CD | US-18, US-19, US-20 | 21 |

Velocidade média (sprints 1–4): **~15 pts/sprint**.

## Sprint 0 — Fundação
* Workspace Cargo com 4 crates (`domain`, `application`, `infrastructure`, `cli`)
  e a regra de dependência imposta pelo `Cargo.toml` de cada crate.
* Pipeline de CI: `fmt`, `clippy -D warnings`, testes.
* Backlog priorizado e Definition of Done.

## Sprint 1 — Domínio
**Incremento:** biblioteca de domínio com 100% das regras testadas sem I/O.
* Entidade `Speaker` com consentimento obrigatório (US-01).
* Fonemas IPA, parser de IPA, palavras e enunciados.
* Modelo de sotaque: regras segmentais, de término e de junção; derivação
  ordenada; alinhamento canônico × ouvido.

**Retrospectiva**
* 👍 Modelar o sotaque como regras ordenadas deixou os testes legíveis
  ("noite" → /nojtʃi/ prova a alimentação entre regras).
* 👎 Juntar término de palavra e junção gerou conflito ("mar azul": o *r*
  caía antes de ligar). **Ação:** contexto `blocked_before_vowel_word`.

## Sprint 2 — Clonagem com sotaque
**Incremento:** `amostra adicionar` + `voz gerar` funcionando com motor local.
* Decodificação WAV, análise DSP (pitch por autocorrelação, pausas).
* G2P pt-BR (forma de citação) com tonicidade, dígrafos, nasais e ditongos.
* `AccentAnalyzer`: mede oportunidades × realizações por fenômeno; descobre
  padrões fora do catálogo; rotula o preset regional mais próximo.
* Porta `VoiceCloningEngine` + adaptador HTTP para motor neural (feature).

**Retrospectiva**
* 👍 Fakes in-memory nas portas deixaram os casos de uso testáveis em ms.
* 👎 Junções mascaravam regras de coda ("os amigos" → z contava como falha de
  s → ʃ). **Ação:** evidência "explicada por junção" não conta contra a regra.

## Sprint 3 — Mensagens e agenda
**Incremento:** fluxo completo pela CLI, coberto por testes de aceitação.
* Catálogo de mensagens por tema com personalização pelo nome.
* `falar`, `pronuncia` (com intensidade), `agenda criar/listar/remover/executar`.
* Agendamento com janelas, dias da semana, horário de silêncio e *catch-up*
  único quando o app estava desligado.

**Retrospectiva**
* 👍 Testes de aceitação no binário real espelham os critérios Gherkin.
* 👎 Janela de intervalo começava "fora do relógio" (09:30 em vez de 09:00).
  **Ação:** pular para o início da janela; teste de regressão adicionado.

## Sprint 4 — Tom de voz e Android
**Incremento:** tom de voz escolhido em cada mensagem (CLI e app) e APK
gerado automaticamente pelo CI, com release a cada tag.
* `VoiceTone`: seis estilos + ajuste de ±6 semitons, aplicados **sobre** a
  prosódia natural da pessoa; tom inicial sugerido pelo tema.
* Composition root extraída para `crates/bootstrap` (reuso CLI/Android).
* `crates/mobile`: API JSON testável no host + wrappers JNI à prova de panic.
* App Android em Kotlin (sem regras de negócio) e WorkManager para a agenda.
* Workflow `android.yml`: cargo-ndk (3 ABIs), CLI para Termux, APK
  debug/release, lint, artefatos e GitHub Release em tags `v*`.

**Retrospectiva**
* 👍 A Clean Architecture pagou o investimento: o Android só precisou de uma
  nova camada de apresentação; nenhum caso de uso mudou.
* 👎 O SDK Android não estava disponível no ambiente de desenvolvimento.
  **Ação:** o Kotlin foi verificado compilando contra `android-all` (API 34);
  o APK completo é validado no CI.
