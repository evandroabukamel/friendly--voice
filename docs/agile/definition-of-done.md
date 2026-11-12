# Definition of Done

A story is **done** when:

1. All acceptance criteria are covered by automated tests (domain unit tests,
   use case tests with fakes, or CLI/engine acceptance tests).
2. `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test` and the
   engine's `pytest` pass in CI.
3. The Clean Architecture dependency rule holds (the domain depends on
   nothing; the application only on the domain; infrastructure implements ports).
4. New business rules live in the domain; new I/O lives behind a port.
5. User-facing messages are in Portuguese, clear and actionable; code,
   comments and documentation are in English.
6. Documentation (README/ADR/backlog) is updated when behaviour changes.
7. For voice stories: a person listened to the result with real recordings.
8. Reviewed by someone else (PR) before merging.

## Definition of Ready
* Story in the *As a… I want… so that…* format, with acceptance criteria.
* Estimated by the team and small enough to fit in a sprint.
* External dependencies (e.g. the voice server, model licenses) identified.
