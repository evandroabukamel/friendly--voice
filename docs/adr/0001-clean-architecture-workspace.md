# ADR 0001 — Clean Architecture with one crate per layer

**Status:** accepted · Sprint 0

## Context
We must follow Clean Architecture and SOLID, and be able to swap voice engines
and storage without rewriting business rules.

## Decision
A Cargo workspace with `domain`, `application`, `infrastructure` and
presentation crates. Dependencies between crates make the dependency rule
**checked by the compiler**. Manual dependency injection with `Arc<dyn Trait>`
in a single composition root.

## Consequences
* + Layer violations become compile errors.
* + Use cases are tested with fakes, without I/O.
* − Some constructor boilerplate.
