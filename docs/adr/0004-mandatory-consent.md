# ADR 0004 — Consentimento como invariante de domínio

**Status:** aceito · Sprint 1

## Contexto
Clonar a voz de terceiros tem implicações éticas e legais (LGPD: dado
biométrico/sensível).

## Decisão
`Speaker` não pode existir sem um registro de `Consent` (quem autorizou e
quando). A regra fica no domínio, e não na UI, para valer em qualquer
interface (CLI, app, API). Os dados ficam locais por padrão.

## Consequências
* + Impossível cadastrar uma voz sem consentimento por qualquer caminho.
* Próximo passo: revogação apagando amostras e voz (US-15).
