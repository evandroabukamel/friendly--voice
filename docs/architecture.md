# Architecture

## Layers (Clean Architecture)

```
┌─────────────────────────────────────────────────────────────┐
│ cli / mobile  (presentation)   bootstrap (composition root) │
│  ┌───────────────────────────────────────────────────────┐  │
│  │ infrastructure  (adapters: WAV, DSP, G2P, JSON,       │  │
│  │                  HTTP, catalog, delivery, clock)      │  │
│  │  ┌─────────────────────────────────────────────────┐  │  │
│  │  │ application  (use cases + ports/traits)         │  │  │
│  │  │  ┌───────────────────────────────────────────┐  │  │  │
│  │  │  │ domain  (entities, rules, accent, tone)   │  │  │  │
│  │  │  └───────────────────────────────────────────┘  │  │  │
│  │  └─────────────────────────────────────────────────┘  │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
          dependencies always point inwards →
```

The **dependency rule** is enforced by the compiler: each layer is a crate,
the `Cargo.toml` of `domain` depends on no other crate of the project, and
`application` depends only on `domain`.

| Crate | Responsibility | Depends on |
|-------|----------------|-----------|
| `friendly-voice-domain` | Entities, value objects, domain services (accent, tone, schedules) | `chrono`, `serde`, `thiserror` |
| `friendly-voice-application` | Use cases and **ports** (traits) | `domain` |
| `friendly-voice-infrastructure` | Port implementations | `domain`, `application`, `hound`, `serde_json`, (`ureq`) |
| `friendly-voice-bootstrap` | Composition root (dependency wiring) | `domain`, `application`, `infrastructure` |
| `friendly-voice` (cli) | Command line interface | `bootstrap` |
| `friendly-voice-mobile` | Android interface: JNI bridge with JSON in/out | `bootstrap`, `jni` |

The Android app (`android/`, Kotlin) is just another presentation layer:
it calls `NativeBridge` → `crates/mobile` → use cases. No business rule is
rewritten in Kotlin (see ADR 0005).

The voice engine (`engine/`, Python) is an external system behind the ports
`VoiceCloningEngine`, `SpeechSynthesizer`, `Transcriber` and
`PhoneRecognizer`, reached through the HTTP adapter (see ADR 0003 and 0006).

## Voice cloning with accent — data flow

```
 WAV ──► WavDecoder ──► AudioClip ──► SampleQualityPolicy (domain)
                              │
                              ├──► DspAcousticAnalyzer ──► pitch, pauses
 text ──► PortugueseG2p ──► citation form  /lejte kẽte/
 heard phonemes (IPA or PhoneRecognizer)  ──► /lejtʃi kẽtʃi/
                              │
                              ▼
                     AccentAnalyzer (domain)
       alignment + opportunities × realizations per phenomenon
                              │
           AccentProfile (ordered rules + confidence)
                              │
 new text ─► G2P ─► AccentRenderer ─► words (IPA, stress, pauses) + tone
                                              │
                                   HttpVoiceEngine (adapter)
                                              │
              engine: Piper pt-BR (speaks the phonemes) ─► OpenVoice V2 (timbre) ─► WAV
```

The timbre (voice colour) needs a neural model, so it sits behind the ports.
**The accent is modelled in the domain**, in an explainable and testable way,
and reaches the engine as explicit pronunciation (phonemes per word) — so the
accent is preserved even though neural models tend to pull towards a
standard pronunciation.

## Accent model

Each phenomenon is a `target → replacement / context` rule:

| Requested aspect | Kind | Examples of catalog rules |
|---|---|---|
| How certain letters are pronounced | `Segmental` | `t`→`tʃ` before /i/, coda `s`→`ʃ`, coda `ɾ`→`h`/`ɻ`, `x`→`h`/`r`, `ʎ`→`j`, open pretonic vowels |
| Word endings | `WordEnding` | unstressed final `e`→`i` and `o`→`u`, `a`→`ɐ`, `e`→`ɨ`, dropping final `r` |
| Word junctions | `Juncture` | `s`+vowel→`z`, `r` linking (tap), `a` elision, `i`→glide, `s`+`s` degemination |

Contexts: position (initial, final, coda, onset, final vowel, pretonic),
previous/next phoneme (exact, set or natural class), stress, and blocking
before a vowel-initial word.

**Learning** (`AccentAnalyzer`):
1. Aligns (weighted Levenshtein) the citation form with the heard phonemes.
2. Walks the catalog **in order**; for each rule it counts opportunities
   (context satisfied) and realizations (the person actually produced the
   replacement). Realizations are applied to the derivation, feeding the
   following rules ("noite": `e→i` creates the context for `t→tʃ`).
3. Evidence masked by a juncture does not penalize word-final rules.
4. Adopts rules with ≥ 2 opportunities and a rate ≥ 50%; the rate is kept as
   confidence (used by the accent **intensity**).
5. Discovers recurring substitutions outside the catalog (`learned:v>b@WordInitial`).
6. Labels the profile with the closest regional preset (Jaccard similarity).

## Tone of voice

`VoiceTone` (domain) = style (`ToneStyle`) + pitch shift in semitones. Each
style is a `ToneAdjustment` (semitones, rate factor, melodic expressiveness,
volume, breathiness) applied **on top of** the person's natural prosody — the
voice is still theirs, only its colour changes. The message theme suggests the
initial tone (`ToneStyle::default_for`), which the user can override on every
request or schedule.

## SOLID in practice

* **S** — one use case per file (`RegisterSpeaker`, `AddVoiceSample`,
  `BuildVoiceProfile`, `SpeakMessage`, ...); accent analysis, rendering and
  alignment in separate modules.
* **O** — new accents = new catalog rules (data), without changing the
  analyzer; new engines/storage = new adapters.
* **L** — any `SpeechSynthesizer` implementation (local sketch, HTTP engine,
  test fake) is interchangeable.
* **I** — small ports: `Transcriber` and `PhoneRecognizer` are optional and
  separate; one repository per aggregate.
* **D** — use cases receive `Arc<dyn Port>`; only the composition root
  (`crates/bootstrap`) knows concrete types — and it is the same for CLI and Android.
