# CLAUDE.md — Friendly Voice

Positive, motivational messages spoken **in the voice of someone you love**,
on demand or on a schedule. Rust core (Clean Architecture), a Python
voice-cloning server, a CLI and a Flutter app (Android + iOS).

## Where to look

- **Working in `flutter_app/`** → [`flutter_app/CLAUDE.md`](flutter_app/CLAUDE.md)
  (UI product rules, bridge rules, packages).
- **Working in `engine/` or the HTTP adapter** → [`engine/CLAUDE.md`](engine/CLAUDE.md)
  (which engine runs, endpoints, gotchas, licences).
- **A limit, a CLI env var, or the data folder** → [`docs/limits.md`](docs/limits.md).
- **Layers, data flow, cache, US-15 deletion, listener name, tone** →
  [`docs/architecture.md`](docs/architecture.md); HTTP contract →
  [`docs/engine-api.md`](docs/engine-api.md).
- **A story's acceptance criteria** → [`docs/agile/product-backlog.md`](docs/agile/product-backlog.md);
  sprint history → [`docs/agile/sprints.md`](docs/agile/sprints.md);
  decisions → [`docs/adr/`](docs/adr).

## Language rules (non-negotiable)

- **Interface, code, comments, docs, commits, ADRs are English** (US-21,
  US-25, US-26). The former Portuguese command/option names stay as hidden
  clap aliases (`pessoa adicionar --nome`), covered by an acceptance test.
- Themes and tones accept English or Portuguese values (`warm`/`carinhoso`).
- The **spoken** message catalog is Portuguese (`infrastructure/src/messages.rs`).
- Error messages are clear and actionable ("split it into pieces").

## Architecture rules (ADR 0001)

```
crates/domain          pure rules; depends on no project crate (chrono, serde, thiserror only)
crates/application     use cases (one per file) + ports (traits); depends only on domain
crates/infrastructure  adapters: WAV/FLAC decode, DSP, JSON repos, HTTP engine, sketch engine, cache
crates/bootstrap       the ONLY composition root, shared by the CLI and the Flutter app
crates/cli             `friendly-voice` binary (clap)
flutter_app/           Flutter app — presentation only (ADR 0009); flutter_app/rust is the bridge
engine/                Python FastAPI server: Chatterbox Multilingual
```

- New business rules go in `domain`; new I/O goes behind a port in
  `application/src/ports.rs` and is wired in `bootstrap`. Application and
  domain import only inward.
- Use cases take `Arc<dyn Port>`; tests use the in-memory fakes in
  `application/src/fakes.rs`.
- Audio format is detected by **magic bytes** (`RIFF…WAVE` / `fLaC`), never by
  extension. FLAC becomes WAV before the engine: the API contract is WAV-only.
- `domain/src/language.rs` (`SUPPORTED`) must match
  `engine/friendly_voice_engine/languages.py`.
- Stored JSON stays readable: new fields get `#[serde(default)]`, old fields
  are ignored (compatibility test in `domain/src/voice.rs`).
- The neural model imitates timbre, intonation, rhythm and accent from the
  person's own recordings (ADR 0007). The withdrawn accent rules, G2P, phone
  recognizer, pronunciation preview and accent intensity (US-03/04/06/09/10)
  stay out.

## Ethics (ADR 0004)

Every person is registered **with recorded consent**; revoking it deletes
their samples, voice and cached speech (US-15). Personal, private use only;
data stays local (data folder + engine voices folder).

## Commands

```bash
cargo build --release --features http-engine    # binary: target/release/friendly-voice(.exe); README aliases it as `fv`
cargo test --workspace --all-features           # unit, use-case and acceptance tests (real binary)
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo fmt --all
```

App and engine commands live in their folder's `CLAUDE.md`. CI
(`.github/workflows`): `ci.yml` (fmt, clippy, tests), `engine.yml` (engine
tests with the real model on CPU, e2e, Docker build), `flutter.yml` (bindings
up to date, bridge + Dart tests, Android APK, unsigned iOS; Release on `v*`).

## Definition of Done (summary)

Acceptance criteria covered by automated tests · fmt/clippy/tests/pytest green
· dependency rule holds · English interface/code/docs · README/ADR/backlog
updated when behaviour changes · voice stories checked by listening to real
recordings · reviewed in a PR. Full text:
[`docs/agile/definition-of-done.md`](docs/agile/definition-of-done.md).

<!-- claude-reflect: learned rules (saved from the band; edit freely) -->
## Learned rules
- When lengthening vowels across word boundaries, lengthen only the stressed (tônica) vowel, not unstressed ones.
<!-- end claude-reflect -->
