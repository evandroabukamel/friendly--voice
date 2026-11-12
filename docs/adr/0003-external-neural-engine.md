# ADR 0003 — Timbre via motor neural externo atrás de portas

**Status:** aceito · Sprint 2

## Contexto
Clonar timbre exige modelos neurais pesados (XTTS, OpenVoice, F5-TTS), em
Python/GPU, com licenças próprias.

## Decisão
Definir as portas `VoiceCloningEngine`, `SpeechSynthesizer`, `Transcriber` e
`PhoneRecognizer` e um contrato HTTP simples (`docs/engine-api.md`), com
adaptador Rust opcional (feature `http-engine`). Localmente, um sintetizador de
**esboço de prosódia** valida ritmo/entonação e o fluxo ponta a ponta.

## Consequências
* + O core Rust fica leve, sem dependências de ML.
* + Qualquer modelo pode ser plugado implementando 4 endpoints.
* − Sem o servidor, o áudio local não reproduz o timbre real.
