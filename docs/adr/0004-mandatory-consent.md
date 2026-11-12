# ADR 0004 — Consent as a domain invariant

**Status:** accepted · Sprint 1

## Context
Cloning a third party's voice has ethical and legal implications (in Brazil,
LGPD treats voice as biometric/sensitive data).

## Decision
A `Speaker` cannot exist without a `Consent` record (who authorized it and
when). The rule lives in the domain, not in the UI, so that it holds for every
interface (CLI, app, API). Data stays local by default.

## Consequences
* + It is impossible to register a voice without consent through any path.
* Next step: revoking consent deletes the samples and the voice (US-15).
