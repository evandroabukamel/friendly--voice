# Product Backlog — Friendly Voice

**Product vision:** receive positive, motivational messages spoken in the
voice (and accent) of someone dear — whenever I ask, or at the moments I
choose.

**Main persona:** *Evandro*, who wants to hear his grandmother's voice saying
"bom dia" before work and "respira fundo" in the middle of the afternoon.

Prioritization: **MoSCoW** · Estimation: **story points (Fibonacci)** · Stories
in the *As a… I want… so that…* format, with acceptance criteria in Gherkin.
Spoken messages are in Brazilian Portuguese (the voice and accent model are
pt-BR); the interface is in English. Some criteria written before Sprint 6
quote the former Portuguese interface texts.

---

## Epic E1 — People and consent

### US-01 · Register the person of the voice — *Must* · 2 pts · ✅ Sprint 1
> As a user, I want to register the person whose voice will be used, recording
> their consent, so that the voice is used ethically.

```gherkin
Scenario: registration without consent
  Given I only provide the name "Vó"
  When I try to register
  Then the registration is refused with "é obrigatório registrar o consentimento"

Scenario: registration with consent
  Given I provide the name "Vó" and that "Vó" authorized it
  When I register
  Then I get an id and the consent statement with date/time
```

## Epic E2 — Voice cloning

### US-02 · Upload audio files — *Must* · 5 pts · ✅ Sprint 2
> As a user, I want to upload recordings of the person so that their voice can
> be generated.

```gherkin
Scenario: valid sample
  Given a 6 s, 16 kHz WAV with the transcript "Leite quente de noite"
  When I add the sample
  Then it is stored with duration, mean pitch and pause ratio

Scenario: bad sample
  Given a WAV shorter than 2 s (or clipped, silent, < 16 kHz)
  When I add the sample
  Then it is rejected with an explanation
```

### ~~US-03, US-04, US-06, US-09, US-10~~ · Accent rules — withdrawn in Sprint 7
Learning letter pronunciation, word endings and junctions as rules,
previewing the pronunciation, the accent intensity and off-catalog traits
were delivered in Sprints 2–3 and withdrawn when the synthesis moved to
zero-shot cloning, which imitates the accent from the recordings themselves
(ADR 0007).

### US-11 · Clone the voice from the uploaded recordings — *Must* · 8 pts · ✅ Sprint 5, redone in Sprint 7
> As a user, I want the messages to sound like the person — timbre,
> intonation, rhythm and accent — and to know whether it is a female or male voice.

```gherkin
Scenario: clone from every recording
  Given the engine server is running with Chatterbox Multilingual (MIT)
  When I generate the voice of "Vó" from her recordings
  Then the speaker embedding is the average of all recordings
  And the reference is the 10 s window closest to that average
  And the profile says "female voice" when the mean pitch is ≥ 160 Hz

Scenario: speak with the voice and the tone
  Given the voice of "Vó" was generated
  When I ask for "Bom dia" with tone "warm"
  Then the engine receives the text, the language and the tone
  And returns a WAV in her voice; a take that was cut off is retried

Scenario: optional transcription
  Given the server has no transcriber installed
  When I add a sample without a transcript
  Then the sample is stored without one
```

### US-27 · Choose the language of the person — *Should* · 3 pts · ✅ Sprint 7
> As a user, I want to say which language the person speaks in the
> recordings, so that the voice speaks my messages in that language.

```gherkin
Scenario: language per person
  When I register "Grandpa" with --language en-US
  Then samples are transcribed and messages spoken in English
  And an unsupported language is rejected with the list of supported ones

Scenario: themes are Portuguese
  Given a person who speaks English
  When I ask for a theme
  Then I am told to pass the text instead
```

### US-24 · Upload recordings in FLAC — *Should* · 2 pts · ✅ Sprint 6
> As a user, I want to upload FLAC recordings (lossless, common in phone and
> studio recorders) as well as WAV.

```gherkin
Scenario: FLAC sample
  Given a 6 s FLAC recording (any bit depth, mono or stereo) with its .txt/.ipa files
  When I add the sample
  Then it is decoded, analyzed and stored as .flac, like a WAV sample

Scenario: cloning with FLAC references
  Given the voice engine is configured
  When I generate the voice from FLAC samples
  Then the references are converted to WAV before being sent (the API contract is unchanged)

Scenario: unsupported format
  When I add an OGG file
  Then I am told that WAV (PCM/float) and FLAC are supported
```

### US-25 · CLI commands in English — *Should* · 2 pts · ✅ Sprint 6
> As a user, I want the CLI commands and options in English
> (`person add --name`), without breaking my existing scripts.

```gherkin
Scenario: English commands and values
  When I run "person add --name Grandma --consent-by Grandma"
  And "speak --theme good-morning --tone whisper"
  Then they work as the former Portuguese commands did

Scenario: backwards compatibility
  When I run the former "pessoa adicionar --nome Vó --consentimento-de Vó"
  Then it still works
```

### US-26 · Interface output in English — *Should* · 3 pts · ✅ Sprint 6
> As a user, I want the help, messages, errors and reports of the CLI and the
> Android app in English, while the spoken messages stay in Portuguese.

```gherkin
Scenario: English output
  When I run "person add --name Grandma"
  Then I get "the consent of 'Grandma' must be recorded before cloning the voice"

Scenario: English names for themes and tones
  When I ask for a message of the theme "calm"
  Then the output shows "tone: calm"
  And the Portuguese names ("calma", "calmo") are still accepted on input
```

## Epic E3 — Messages

### US-05 · Ask for a message right now — *Must* · 3 pts · ✅ Sprint 3
> As a user, I want to ask for a message now (by theme or free text) and hear
> it in the person's voice.

```gherkin
Scenario: personalized message by theme
  Given the voice of "Vó" was generated and my name is "Evandro"
  When I ask for a message of the theme "bom-dia"
  Then I get a WAV with a message that contains "Evandro"

Scenario: voice not generated yet
  When I ask for a message
  Then I am told to upload samples and generate the voice
```

### US-18 · Choose the tone of voice of each message — *Must* · 5 pts · ✅ Sprint 4
> As a user, I want to choose the tone of voice (natural, carinhoso, animado,
> calmo, firme, sussurrado) and make the voice higher or lower for each
> message — on demand or scheduled — so that it fits the moment.

```gherkin
Scenario: initial tone suggested by the theme
  When I ask for a message of the theme "calma" without a tone
  Then it is spoken in the tone "calmo"

Scenario: the user's choice
  When I ask for a message of the theme "calma" with tone "animado" and -2 semitones
  Then it is spoken in the tone "animado", 2 semitones lower
  And the person's natural prosody (learned from the samples) is kept as the base

Scenario: tone in a schedule
  Given a daily schedule with tone "sussurrado"
  When it fires
  Then the message is whispered

Scenario: invalid values
  When I provide the tone "bravo" or a 9-semitone shift
  Then I get an error explaining the valid options
```

## Epic E4 — Schedules

### US-07 · Schedule messages with rules — *Must* · 5 pts · ✅ Sprint 3
> As a user, I want to schedule messages with rules (daily, weekdays, every N
> minutes inside a window, once) and quiet hours.

```gherkin
Scenario: interval with window and quiet hours
  Given the rule "every 60 min between 09:00-18:00" and quiet hours "12:00-14:00"
  When I compute the firings from 10:00
  Then they happen at 11:00, 14:00, ..., 17:00 and then 09:00 the next day

Scenario: impossible rule
  Given the rule "every day at 08:00" with quiet hours "00:00-23:59"
  Then the schedule is refused ("nunca dispara")
```

### US-08 · Fire the schedules that are due — *Must* · 3 pts · ✅ Sprint 3
> As a user, I want due messages to be spoken exactly once, even if the app was
> off.

## Epic E5 — Android

### US-19 · Build the Android app in CI/CD — *Must* · 8 pts · ✅ Sprint 4
> As a maintainer, I want every push to build the Android app (APK) from the
> same Rust core and every tag to publish a release, so that the app can be
> installed and distributed without manual steps.

```gherkin
Scenario: build on every push
  When I push to any branch
  Then CI builds the Rust library for arm64-v8a, armeabi-v7a and x86_64
  And builds the debug and release APKs
  And publishes APKs, native libraries and the lint report as artifacts

Scenario: continuous delivery
  When I create the tag "v1.2.0"
  Then a GitHub Release "v1.2.0" is published with the APKs (versionName 1.2.0)
  And the APK is signed if the keystore secrets are configured
```

### US-20 · Use the app on the phone — *Should* · 8 pts · ✅ Sprint 4
> As a user, I want to register the person, upload recordings, generate the
> voice, ask for messages (choosing the tone) and schedule them on my phone,
> getting a notification and audio when a scheduled message arrives.

## Epic E6 — Project

### US-21 · Documentation and code comments in English — *Should* · 3 pts · ✅ Sprint 5
> As a contributor, I want the README, the documentation and the code comments
> in English, so that people outside Brazil can contribute. The product's user
> interface stays in Portuguese.

---

## Next stories (future refinement)

| Id | Story | Priority | Pts |
|----|-------|----------|-----|
| US-14 | ~~Mobile app with notifications using the use cases~~ → delivered on Android (US-19/US-20) | — | — |
| US-15 | Revoking consent deletes the person's samples and voice (app and engine) | Must | 3 |
| US-16 | ~~FLAC support on import~~ → delivered (US-24); MP3/OGG/M4A still pending | Should | 2 |
| US-17 | Avoid repeating the same message in a row (history) | Could | 2 |
| US-22 | ~~Inaudible watermark on cloned audio~~ → every Chatterbox output carries the Perth watermark (Sprint 7) | — | — |
| US-23 | ~~Base voice closest to the person~~ → no base voice since zero-shot cloning (ADR 0007) | — | — |
| US-28 | Message themes written in other languages | Could | 3 |
| US-29 | First-person words agreeing with the speaker's gender ("obrigado"/"obrigada") | Could | 2 |
