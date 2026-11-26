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
* Revoking consent (US-15, Sprint 8) is also a domain rule: the person is
  kept as a revoked record (`consent_revoked: {by, at}`), every use of the
  voice is refused, and the recordings, the voice (locally and on the voice
  server, `DELETE /v1/voices/{id}`) and the cached speech are deleted. The
  operation is idempotent and a re-run finishes an interrupted cleanup.
  Using the person again requires a new registration with a new consent.
